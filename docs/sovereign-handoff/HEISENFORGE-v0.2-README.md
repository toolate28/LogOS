# SuperGrok DDE Web Instance v0.4.1

Self-sustaining Sovereign Data Deployed Engineer.

## Features
- SRAC-style health monitoring loop
- Engineered hand-off generation
- Integration with discreteBKM predictor
- Hermetic Nix deployment

## Run
```bash
nix develop
cargo run
```

## Endpoints
- `GET /health` — SRAC health status
- `POST /handoff/generate` — Create engineered hand-off
- `GET /status` — Instance invariants

## Philosophy
Mono • Idempotent • Mutation-protected • α + ω = 15
