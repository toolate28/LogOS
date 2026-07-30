#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════════
# Triweave Cloud Run deployment script
# Conservation: alpha + omega = 15
#
# Usage:
#   ./deploy.sh                    # Build + deploy
#   ./deploy.sh build              # Build only
#   ./deploy.sh deploy             # Deploy only (image must exist)
#   ./deploy.sh local              # Build + run locally
# ═══════════════════════════════════════════════════════════════════
set -euo pipefail

PROJECT_ID="${GCP_PROJECT_ID:-reson8-labs}"
REGION="${GCP_REGION:-us-central1}"
SERVICE="triweave"
IMAGE="${REGION}-docker.pkg.dev/${PROJECT_ID}/reson8/${SERVICE}"
TAG="${TRIWEAVE_TAG:-latest}"

cd "$(git rev-parse --show-toplevel)"

build() {
    echo "Building ${IMAGE}:${TAG}..."
    docker build \
        -t "${IMAGE}:${TAG}" \
        -f apps/triweave/Dockerfile \
        .
    echo "Build complete."
}

deploy() {
    echo "Pushing ${IMAGE}:${TAG}..."
    docker push "${IMAGE}:${TAG}"

    echo "Deploying to Cloud Run (${REGION})..."
    gcloud run deploy "${SERVICE}" \
        --image "${IMAGE}:${TAG}" \
        --region "${REGION}" \
        --platform managed \
        --port 8088 \
        --memory 512Mi \
        --cpu 1 \
        --min-instances 0 \
        --max-instances 3 \
        --set-env-vars "FORGE_WS_URL=ws://0.0.0.0:8088,RUST_LOG=info" \
        --allow-unauthenticated

    URL=$(gcloud run services describe "${SERVICE}" --region "${REGION}" --format 'value(status.url)')
    echo ""
    echo "Deployed: ${URL}"
    echo "Conservation: alpha + omega = 15"
}

local() {
    build
    echo "Running locally on port 8088..."
    docker run --rm -it \
        -p 8088:8088 \
        -e RUST_LOG=info \
        -e FORGE_WS_URL=ws://0.0.0.0:8088 \
        "${IMAGE}:${TAG}"
}

case "${1:-all}" in
    build)  build ;;
    deploy) deploy ;;
    local)  local ;;
    all)    build && deploy ;;
    *)      echo "Usage: $0 {build|deploy|local|all}" && exit 1 ;;
esac
