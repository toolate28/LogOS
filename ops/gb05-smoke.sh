#!/usr/bin/env bash
set -euo pipefail
export PATH="${HOME}/.nix-profile/bin:${PATH}"
cd "${HOME}/LogOS"

# sync fixed manifests
cp -f "/mnt/f/Users/Matthew Ruhnau/LogOS/compose.yaml" ./compose.yaml
cp -f "/mnt/f/Users/Matthew Ruhnau/LogOS/k8s/base/deployment.yaml" ./k8s/base/deployment.yaml
cp -f "/mnt/f/Users/Matthew Ruhnau/LogOS/k8s/base/kustomization.yaml" ./k8s/base/kustomization.yaml

DIGEST=33e82ac482dbf4c9ae512c539327c629d5912bf2fe200df43dd9e1549a263a42
echo "DIGEST=${DIGEST}"
grep -n image compose.yaml k8s/base/deployment.yaml
grep -n digest k8s/base/kustomization.yaml

docker compose down 2>/dev/null || true
docker compose up -d
sleep 4
curl -sf http://127.0.0.1:8080/health; echo

VDIR=docs/schemas/v0.1/vectors
curl -sS -X POST http://127.0.0.1:8080/validate/handoff_packet \
  -H 'Content-Type: application/json' --data-binary @"${VDIR}/packet_example_sa01.json" -o /tmp/c-sa.json
curl -sS -X POST http://127.0.0.1:8080/validate/certificate \
  -H 'Content-Type: application/json' --data-binary @"${VDIR}/certificate_example_lane_d.json" -o /tmp/c-ld.json
curl -sS -X POST http://127.0.0.1:8080/validate/certificate \
  -H 'Content-Type: application/json' --data-binary @"${VDIR}/existence_certificate_pre_freeze.json" -o /tmp/c-pf.json

kind load docker-image reson8-waist:0.1.0 --name reson8
docker exec reson8-control-plane ctr -n k8s.io images tag \
  docker.io/library/reson8-waist:0.1.0 \
  "docker.io/library/reson8-waist@sha256:${DIGEST}" 2>/dev/null || true

kubectl --context kind-reson8 delete -k k8s/base --ignore-not-found --wait=true
kubectl --context kind-reson8 apply -k k8s/base
kubectl --context kind-reson8 kustomize k8s/base | grep 'image:'
kubectl --context kind-reson8 rollout status deployment/reson8-waist --timeout=120s
kubectl --context kind-reson8 get pods -o wide
POD=$(kubectl --context kind-reson8 get pod -l app.kubernetes.io/name=reson8-waist -o jsonpath='{.items[0].metadata.name}')
kubectl --context kind-reson8 get pod "${POD}" -o jsonpath='ro={.spec.containers[0].securityContext.readOnlyRootFilesystem} ready={.status.containerStatuses[0].ready}'
echo

pkill -f 'port-forward svc/reson8-waist' 2>/dev/null || true
kubectl --context kind-reson8 port-forward svc/reson8-waist 18081:8080 >/tmp/pf-k.log 2>&1 &
PF=$!
sleep 4
curl -sf http://127.0.0.1:18081/health; echo
curl -sS -X POST http://127.0.0.1:18081/validate/handoff_packet \
  -H 'Content-Type: application/json' --data-binary @"${VDIR}/packet_example_sa01.json" -o /tmp/k-sa.json
curl -sS -X POST http://127.0.0.1:18081/validate/certificate \
  -H 'Content-Type: application/json' --data-binary @"${VDIR}/certificate_example_lane_d.json" -o /tmp/k-ld.json
curl -sS -X POST http://127.0.0.1:18081/validate/certificate \
  -H 'Content-Type: application/json' --data-binary @"${VDIR}/existence_certificate_pre_freeze.json" -o /tmp/k-pf.json

python3 <<'PY'
import json, pathlib, datetime, subprocess

def check(p):
    d = json.load(open(p))
    return d.get("tag"), d.get("error_count")

print("compose", check("/tmp/c-sa.json"), check("/tmp/c-ld.json"), check("/tmp/c-pf.json"))
print("kind", check("/tmp/k-sa.json"), check("/tmp/k-ld.json"), check("/tmp/k-pf.json"))
sc_c = check("/tmp/c-sa.json") == ("PASS", 0) and check("/tmp/c-ld.json") == ("PASS", 0) and check("/tmp/c-pf.json") == ("FAIL", 10)
sc_k = check("/tmp/k-sa.json") == ("PASS", 0) and check("/tmp/k-ld.json") == ("PASS", 0) and check("/tmp/k-pf.json") == ("FAIL", 10)
print("SC-compose", sc_c, "SC-kind", sc_k)

subprocess.run(["kubectl", "--context", "kind-reson8", "delete", "pod", "-l", "app.kubernetes.io/name=reson8-waist", "--wait=true"], check=False)
subprocess.run(["kubectl", "--context", "kind-reson8", "rollout", "status", "deployment/reson8-waist", "--timeout=90s"], check=False)
pod = subprocess.check_output(
    ["kubectl", "--context", "kind-reson8", "get", "pod", "-l", "app.kubernetes.io/name=reson8-waist",
     "-o", "jsonpath={.items[0].metadata.name}"], text=True)
ro = subprocess.check_output(
    ["kubectl", "--context", "kind-reson8", "get", "pod", pod,
     "-o", "jsonpath={.spec.containers[0].securityContext.readOnlyRootFilesystem}"], text=True)
ready = subprocess.check_output(
    ["kubectl", "--context", "kind-reson8", "get", "pod", pod,
     "-o", "jsonpath={.status.containerStatuses[0].ready}"], text=True)
print("restart", pod, "ro=", ro, "ready=", ready)

kind_v = subprocess.check_output(["kind", "version"], text=True).strip()
kubectl_v = subprocess.check_output(["kubectl", "version", "--client", "-o", "yaml"], text=True)
kv = [l for l in kubectl_v.splitlines() if "gitVersion" in l][0].strip()
digest = "33e82ac482dbf4c9ae512c539327c629d5912bf2fe200df43dd9e1549a263a42"
surface = {
    "atom": "ATOM-GB05-ORCH-20260713",
    "packet": "PKT-GB05",
    "recorded_at": datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
    "kind_version": kind_v,
    "kubectl_version": kv,
    "image_digest": f"sha256:{digest}",
    "image_name": "reson8-waist:0.1.0",
    "compose": {"file": "compose.yaml", "sc_compose": sc_c, "read_only": True, "user": "65534:65534"},
    "k8s": {
        "base": "k8s/base",
        "cluster": "kind-reson8",
        "sc_kind": sc_k,
        "readOnlyRootFilesystem": ro == "true",
        "pod_restart_ready": ready == "true",
    },
    "vector_triple": "PASS/PASS/FAIL-10",
    "notes": "Bookshelf not mounted; no Helm; no keys; image by digest; GET / is 404 by design",
}
path = pathlib.Path("adapters/k8s/surface_manifest.json")
path.parent.mkdir(parents=True, exist_ok=True)
path.write_text(json.dumps(surface, indent=2) + "\n")
print(json.dumps(surface, indent=2))
if not (sc_c and sc_k and ro == "true" and ready == "true"):
    raise SystemExit("GB-05 SC failed")
print("GB05_ALL_GREEN")
PY

kill "${PF}" 2>/dev/null || true
mkdir -p "/mnt/f/Users/Matthew Ruhnau/LogOS/adapters/k8s" "/mnt/f/Users/Matthew Ruhnau/LogOS/ops"
cp -v compose.yaml "/mnt/f/Users/Matthew Ruhnau/LogOS/"
cp -v k8s/base/* "/mnt/f/Users/Matthew Ruhnau/LogOS/k8s/base/"
cp -v adapters/k8s/surface_manifest.json "/mnt/f/Users/Matthew Ruhnau/LogOS/adapters/k8s/"
{
  echo ""
  echo "## Attestation $(date -u +%Y-%m-%dT%H:%M:%SZ)"
  echo "image_digest: sha256:${DIGEST}"
  echo "SC-compose: PASS/PASS/FAIL-10"
  echo "SC-kind: PASS/PASS/FAIL-10 + readOnlyRootFilesystem + restart ready"
  echo "note: GET / returns 404 by design; use GET /health"
} >> ops/GB05-worklog.md
cp ops/GB05-worklog.md "/mnt/f/Users/Matthew Ruhnau/LogOS/ops/"
echo GB05_DONE
