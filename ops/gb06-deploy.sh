#!/usr/bin/env bash
# GB-06 Cloud Run deploy helper — PKT-GB06 / ATOM-GB06-CLOUDRUN-20260713
# ⚑ Requires: gcloud auth, project, billing, Artifact Registry.
# NEVER embeds private keys. IAM-gated invoker only.
set -euo pipefail

IMAGE_DIGEST="${IMAGE_DIGEST:-sha256:88b870e3011605d36d6d23bdd56c8b254e4bb1606168e700299a3e4c19965d6b}"
REGION="${REGION:-australia-southeast1}"
SERVICE="${SERVICE:-reson8-waist}"
PROJECT="${PROJECT:-${GOOGLE_CLOUD_PROJECT:-}}"
AR_REPO="${AR_REPO:-reson8}"

if [[ -z "${PROJECT}" ]]; then
  echo "Set PROJECT or GOOGLE_CLOUD_PROJECT" >&2
  exit 2
fi

HOST="${REGION}-docker.pkg.dev"
IMAGE_PATH="${HOST}/${PROJECT}/${AR_REPO}/reson8-waist@${IMAGE_DIGEST}"

echo "GB-06 deploy"
echo "  project=${PROJECT} region=${REGION}"
echo "  image=${IMAGE_PATH}"
echo "  allow-unauthenticated=false (IAM invoker only)"

gcloud auth configure-docker "${HOST}" --quiet
gcloud run deploy "${SERVICE}" \
  --project="${PROJECT}" \
  --region="${REGION}" \
  --image="${IMAGE_PATH}" \
  --no-allow-unauthenticated \
  --port=8080 \
  --min-instances=0 \
  --cpu=1 \
  --memory=512Mi \
  --set-env-vars="PORT=8080,SCHEMAS_DIR=/schemas"

echo "Deploy requested. Smoke with identity token:"
echo "  TOKEN=\$(gcloud auth print-identity-token)"
echo "  curl -H \"Authorization: Bearer \$TOKEN\" https://SERVICE_URL/health"
