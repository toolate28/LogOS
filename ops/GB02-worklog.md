# GB-02 Work Log — ATOM-GB02-WAIST-IMG-20260713
SELF-WITNESS:
context_depth: GB-00+01 closed; schemas v0.1 filed; docker 28.5; flake pin e7a3ca8
drift_count: 0
unknowns: [self-hash canonical serialization for md5-legacy vectors; dockerTools build time]

## Attestation 2026-07-14T12:40Z
SC-vectors: PASS/PASS/FAIL-10
SC-stateless: docker run --read-only + /health ok
SC-manifest: schema sha256 match tree
image_id: sha256:4b774a0b69fa11651f104865c909a0b547482fa5677e7fb1415d1262173dc091
nix_out: /nix/store/6sd1wkkc76af1imdh0rbckhdv2q81fsk-reson8-waist.tar.gz
exit_codes: docker_load=0 docker_run=0 curl_vectors=0
refuse_/emit: 404 by design
6461d091bb3ed0e0dc71a568e1072d32e3b6beb7d00eb8fc6a0c9164c478a5c7  services/waist/app.py
627b4664b954cd1be6966c0714c143ca47307f53cd6e2a52b6dafa299a2209eb  adapters/waist/surface_manifest.json
7f8889f3f7e78aebb7dfe156fed7dd1221d5a64853574cf9547f195314ab0498  flake.nix
