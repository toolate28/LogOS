# GB-06 Work Log — Cloud Run deploy (PKT-GB06 / ATOM-GB06-CLOUDRUN-20260713)

SELF-WITNESS:
context_depth: GB-00..05 + GB-03/04 closed enough for deploy spine; image sha256:88b870e3…; gcloud absent on host
drift_count: 0
unknowns: [GCP project id, billing, region (au-southeast?), Artifact Registry, SAs]

## Status
**BLOCKED on ⚑ human GCP prerequisites.** No private keys in cloud. Verify-only.

## Image (same as compose/kind)
```
reson8-waist:0.1.0
sha256:88b870e3011605d36d6d23bdd56c8b254e4bb1606168e700299a3e4c19965d6b
# or rebuild: cd ~/LogOS && nix build .#waist-image --out-link result-waist && docker load < result-waist
```

## ⚑ Human checklist (do before agent deploy)

1. **GCP project** with billing enabled  
2. **Region** choice (suggest `australia-southeast1` if AU residency)  
3. **Artifact Registry** docker repo, e.g. `reson8`  
4. **Deploy SA** roles: `roles/run.admin` + `roles/artifactregistry.writer`  
5. **Runtime SA** role on receipts bucket: `roles/storage.objectCreator` **only** (no delete/overwrite)  
6. **Bucket**: versioning ON + retention policy (append-only posture)  
7. Install / auth gcloud on WSL:
   ```bash
   # then:
   gcloud auth login
   gcloud auth application-default login
   gcloud config set project PROJECT_ID
   gcloud auth configure-docker REGION-docker.pkg.dev
   ```

## Deploy sketch (agent runs after ⚑)

```bash
IMG=REGION-docker.pkg.dev/PROJECT/REPO/reson8-waist@sha256:88b870e3011605d36d6d23bdd56c8b254e4bb1606168e700299a3e4c19965d6b
# tag & push from local docker image id
docker tag sha256:88b870e3011605d36d6d23bdd56c8b254e4bb1606168e700299a3e4c19965d6b $IMG
docker push $IMG

gcloud run deploy reson8-waist \
  --image $IMG \
  --region REGION \
  --port 8080 \
  --no-allow-unauthenticated \
  --min-instances 0 \
  --service-account RUNTIME_SA@PROJECT.iam.gserviceaccount.com
```

### Success criteria
- **SC-remote-vectors:** PASS/PASS/FAIL-10 via identity token  
- **SC-iam:** unauth → 403; invoker token → 200  
- **SC-receipt-loop:** POST /receipt → GCS object → local `gsutil rsync` pull with hash intact  

### Absolute DO NOTs
- `--allow-unauthenticated` in v0  
- private keys in image/env/Secret Manager  
- runtime SA bucket delete  
- point Museum emit at cloud URL  

## Adapter
`adapters/gcloud/surface_manifest.json` — TEMPLATE-AWAITING-GCP
