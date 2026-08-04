# GCP · Gemini for Google Cloud — LogOS inventory

**ATOM:** `ATOM-GCP-GEMINI-ASSIST-20260804`  
**Epistemic:** Category B inventory (operator-supplied + API name map)  
**capability ≠ authority**

## Identities

| Principal | Role |
|-----------|------|
| `matthew.ruhnau@gmail.com` | **Owner** |
| `service-952127156617@gcp-sa-cloudaicompanion.iam.gserviceaccount.com` | **Gemini for Google Cloud Service Agent** (`roles/cloudaicompanion.serviceAgent` typical) |

| Field | Value |
|-------|--------|
| Project **number** | `952127156617` |
| Project **ID** | *re-resolve with gcloud* (`gcloud projects describe 952127156617 --format='value(projectId)'`) |
| Service agent pattern | `service-PROJECT_NUMBER@gcp-sa-cloudaicompanion.iam.gserviceaccount.com` |

> Do **not** commit user access tokens or `application_default_credentials.json`.  
> Owner email is identity metadata, not a secret.

## Enabled APIs (operator timeline)

### Gemini / AI surface (recent)

| API | Purpose |
|-----|---------|
| `geminicloudassist.googleapis.com` | Gemini Cloud Assist |
| `cloudaicompanion.googleapis.com` | Gemini for Google Cloud (Companion) — **requires service agent above** |
| `aiplatform.googleapis.com` | Vertex AI / Gemini model APIs |
| `designcenter.googleapis.com` | Design Center (infra/app design assist) |
| `appoptimize.googleapis.com` | App Optimize recommendations |
| `apphub.googleapis.com` | App Hub application grouping |
| `apptopology.googleapis.com` | App topology graphs for assist |
| `cloudasset.googleapis.com` | Cloud Asset Inventory (context for assist) |
| `recommender.googleapis.com` | Recommenders consumed by assist |

### Observability / control plane

| API | Purpose |
|-----|---------|
| `logging.googleapis.com` | Cloud Logging |
| `monitoring.googleapis.com` | Cloud Monitoring |
| `iam.googleapis.com` | IAM (enabled earlier) |

## What this enables (and does not)

**Enables**

- Gemini Cloud Assist / Companion in Google Cloud Console for this project  
- Vertex AI (`aiplatform`) model calls when quotas + billing allow  
- Asset/topology/recommender context for “explain my environment” style agents  
- Logging + Monitoring for assist audit trails  

**Does not automatically enable**

- Firebase **App Hosting** backends (needs `firebasehosting` / App Hosting product setup + framework app root)  
- GitHub Actions green builds (repo `sha_pinning_required` is independent)  
- Public LogOS Pages badges (GitHub Pages / `docs/badges`)  
- Silent Category A proofs — Cloud Assist output is **advisory**  

## Required IAM (checklist)

Run when `gcloud` is available (project ID substituted):

```bash
PROJECT_NUMBER=952127156617
PROJECT_ID="$(gcloud projects describe ${PROJECT_NUMBER} --format='value(projectId)')"
SA="service-${PROJECT_NUMBER}@gcp-sa-cloudaicompanion.iam.gserviceaccount.com"

# Confirm APIs
gcloud services list --enabled --project="${PROJECT_ID}" \
  --filter="name:(geminicloudassist OR cloudaicompanion OR aiplatform OR designcenter OR cloudasset OR appoptimize OR apphub OR logging OR monitoring OR apptopology OR recommender OR iam)"

# Confirm service agent exists / role
gcloud projects get-iam-policy "${PROJECT_ID}" \
  --flatten="bindings[].members" \
  --filter="bindings.members:${SA}" \
  --format="table(bindings.role)"

# Owner presence (you)
gcloud projects get-iam-policy "${PROJECT_ID}" \
  --flatten="bindings[].members" \
  --filter="bindings.members:user:matthew.ruhnau@gmail.com" \
  --format="table(bindings.role)"
```

Expected for Companion SA (names may vary by product version):

- `roles/cloudaicompanion.serviceAgent` (or Google-managed equivalent granted at API enable)

Human operator:

- `roles/owner` **or** least-privilege set including browse + AI use as needed  
- For Vertex calls: `roles/aiplatform.user` (if not using Owner)

## Linkage to LogOS CI (GitHub)

| Surface | Owner system | Status intent |
|---------|--------------|---------------|
| SHA-pinned Actions | GitHub `toolate28/LogOS` | `ops/ci/assert_action_pins.py` + `ci-policy.yml` |
| CODEX SARIF + badges | GitHub Code Scanning + `docs/badges` | `codex-mlops.yml` |
| Cloud Gemini config inventory | **This doc** + agent prompt | `docs/ops/CLOUD-GEMINI-AGENT-PROMPT.md` |
| Firebase App Hosting | Firebase/GCP | **Opt-in only** if a Next/Angular app root exists |

Cloud Gemini agent should **IDENTIFY** using this inventory; **ENACT** only when operator says `ENACT`.

## Operator prompt seed (paste into Gemini Cloud Assist)

```text
Project number 952127156617. I am Owner (matthew.ruhnau@gmail.com).
Gemini for Google Cloud Service Agent:
  service-952127156617@gcp-sa-cloudaicompanion.iam.gserviceaccount.com
Enabled: geminicloudassist, cloudaicompanion, aiplatform, designcenter,
  cloudasset, appoptimize, apphub, apptopology, recommender, logging,
  monitoring, iam.

Task: IDENTIFY only — verify these APIs are enabled, confirm the Companion
service agent has the expected role, list any missing APIs for Cloud Assist
+ Vertex, and produce a config matrix. Do not create Firebase App Hosting
backends unless I say ENACT and specify a Next/Angular root directory.
Cross-link: GitHub toolate28/LogOS requires sha_pinning_required Actions pins;
do not claim CI green without checking GitHub Actions runs.
```

## Residual risks

| Risk | Note |
|------|------|
| Project ID unknown in this tree | Only project **number** was supplied; resolve before scripting |
| `gcloud` not on Windows agent host | Inventory is operator-asserted until CLI verifies |
| Billing / quotas | `aiplatform` enable ≠ usable quota |
| Over-broad Owner | Prefer later split of break-glass vs day-to-day AI roles |
| App Hosting mismatch | LogOS monorepo root is not automatically a Next/Angular app |

## Related

- [`CLOUD-GEMINI-AGENT-PROMPT.md`](./CLOUD-GEMINI-AGENT-PROMPT.md)  
- [`AGENTIC-MLOPS-CI.md`](./AGENTIC-MLOPS-CI.md)  
- [`.github/workflows/ci-policy.yml`](../../.github/workflows/ci-policy.yml)  
- [`.github/workflows/codex-mlops.yml`](../../.github/workflows/codex-mlops.yml)  
