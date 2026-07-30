#!/usr/bin/env bash
# GB-00 ⚑ human step — run inside WSL (needs your sudo password once)
# ATOM-GB00-WSL2-NIX-20260713
set -euo pipefail
curl --proto "=https" --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install --no-confirm \
  --extra-conf "experimental-features = nix-command flakes"
# shellenv for current session
if [ -e /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh ]; then
  # shellcheck disable=SC1091
  . /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh
fi
nix --version
# ensure flakes in nix.conf (Determinate usually enables)
if ! grep -q "experimental-features" /etc/nix/nix.conf 2>/dev/null; then
  echo "experimental-features = nix-command flakes" | sudo tee -a /etc/nix/nix.conf
fi
nix run nixpkgs#hello
# refresh manifest
CLONE_SHA=$(git -C ~/LogOS rev-parse HEAD)
NIX_VERSION=$(nix --version)
python3 - << PY
import json, datetime, pathlib
p = pathlib.Path.home() / "LogOS/ops/bootstrap_manifest.json"
m = json.loads(p.read_text())
m["nix_version"] = """$NIX_VERSION""".strip()
m["nix_flakes_enabled"] = True
m["success_criteria"]["SC-nix"] = True
m["recorded_at"] = datetime.datetime.utcnow().strftime("%Y-%m-%dT%H:%M:%SZ")
m["smoke"] = {"nix_run_hello": 0}
p.write_text(json.dumps(m, indent=2) + "\n")
print("manifest updated", p)
PY
cp ~/LogOS/ops/bootstrap_manifest.json "/mnt/f/Users/Matthew Ruhnau/LogOS/ops/bootstrap_manifest.json"
echo "SC-nix GREEN — resume GB-01"
