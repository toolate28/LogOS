# App Hosting · logos-tw1 — 2026-08-15

**ATOM:** `ATOM-APPHOSTING-LOGOS-TW1-20260815`  
**Project:** `tri-weavon` · **backend:** `logos-tw1` · **region:** `us-east4`  
**App root:** `apps/logos-tw1`  
**SDK guide:** https://firebase.google.com/docs/app-hosting/firebase-sdks

The 40s rollout fail is the backend pointed at the **Rust repo root**.
App Hosting needs a framework `package.json`. That now lives here.

## Console — set root + metrics routes

1. Firebase console → App Hosting → logos-tw1 → Settings  
   **App root directory** = `apps/logos-tw1` (must match `firebase.json` `rootDir`).
2. Register metrics routes (max 20) — exact paths:

```
/agda
/apps
/crates
/cutiles
/docs
/kernels
/lean
/notebooks
/ops
/tools
```

Also useful: `/` and `/api/health` and `/api/lattice`.

3. Session you pasted (`czE3ODY3NzI1ODIkbzYkZzEkdDE3ODY3NzMwOTYkajU3JGwwJGgw`) is a
   console session id, not an API key. Do not commit it.

## Firebase SDKs (no-arg init)

App Hosting injects `FIREBASE_CONFIG` (Admin) and `FIREBASE_WEBAPP_CONFIG`
(JS SDK, build-time postinstall). Code uses `initializeApp()` with no
arguments, then a project-id fallback for local `next dev`.

```ts
// lib/firebase.ts — JS SDK
initializeApp();

// lib/firebase-admin.ts — Admin (route handlers only)
initializeApp();
```

Do **not** put API keys in `apphosting.yaml`. Console env vars win over yaml.

## Local

```powershell
cd apps/logos-tw1
npm install
npm run dev
# http://127.0.0.1:3000  ·  /api/health  ·  /api/lattice
```

## CLI (when firebase-tools is on PATH)

```powershell
npx firebase-tools@latest login
npx firebase-tools@latest use tri-weavon
npx firebase-tools@latest apphosting:backends:list --project tri-weavon
# after rootDir is set, next push to the live branch rolls out
```

## Verify jobs this unblocks (separate)

| Job | Cause | Fix |
|-----|--------|-----|
| Tree guards / Secret-path | 160 `lean/.lake/build` files on origin | `git rm -r --cached lean/.lake` (already gitignored) |
| Rust core + wave | `phase.rs` missing on origin | ship `crates/resonance-invariant/src/phase.rs` |
| Formal residual | `UnboundLocalError: os` | `ops/ci/formal_residual_report.py` |
| App Hosting | no Next.js at backend root | this app + console rootDir |
| Pages | GitHub Pages site, not App Hosting | do not treat as logos-tw1 |

capability ≠ authority. α+ω=15 is Category C only.
