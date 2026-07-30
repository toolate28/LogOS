# GB-03 Work Log — ATOM-GB03-STYX-20260713
SELF-WITNESS:
context_depth: GB-00..02+05 green; styx-vfs-layer; v9fs in /proc/filesystems; KVM true
drift_count: 0
unknowns: [full 9P2000.L client edge cases; Inferno optional; kernel mount needs sudo]

## Status: 🟡 NEAR-CLOSED (2026-07-15)

## Design harmony
WSL2 drvfs interop is Plan 9 — Bookshelf promotes host-native 9P to first-class via TCP:5640 v0.
AF_VSOCK is stretch-only; not required for v0.

## Attestation 2026-07-14T17:19:47Z (initial)
cargo build -p styx-vfs-layer: 0
client smoke: hash match + Rerror + VOID
v9fs mount: ⚑ sudo
daemon listen: 127.0.0.1:5640

## Attestation 2026-07-15T06:28:21+10:00 (unit completion)
### SC results
| ID | Result | Notes |
|----|--------|-------|
| SC-client-read-hash | ✅ PASS | schema file over 9P client byte-identical |
| SC-gate | ✅ PASS | disallowed write → Rerror + VOID event in `.atom-trail/` |
| SC-unit | ✅ PASS | systemd-user unit **enabled** + **active (running)** |
| SC-mount | ⚑ BLOCKED | `sudo mount -t 9p ...` needs password; v9fs capable (nodev 9p present) |

### Commands
- `systemctl --user enable --now styx-bookshelf.service` → 0
- `python3 ops/styx-9p-client-smoke.py` → SMOKE_OK
- `ss -ltn` → 127.0.0.1:5640 LISTEN
- `loginctl show-user toolated -p Linger` → Linger=no (enable needs sudo ⚑)
- `sudo -n mount ...` → password required

### Human close-out (one-liner)
```bash
sudo mkdir -p /mnt/bookshelf
sudo mount -t 9p -o trans=tcp,port=5640,version=9p2000.L 127.0.0.1 /mnt/bookshelf
sudo loginctl enable-linger "$USER"
```

### Inferno
optional-skipped-non-blocking

### Artifacts
- `crates/styx-vfs-layer/`
- `ops/styx-bookshelf.service`
- `ops/styx-9p-client-smoke.py`
- `adapters/9p/surface_manifest.json`
