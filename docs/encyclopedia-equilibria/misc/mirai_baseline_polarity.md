# MIRAI Baseline Analysis — Polarity Module

**Date**: 2026-06-28  
**Target**: `src/polarity.rs` + `CutileHarness`

## Attempted Command
```bash
cargo mirai -p cutile -- src/polarity.rs
```

## Current Status
- MIRAI installation via `cargo install mirai` failed (it is distributed as a library, not a standalone binary in recent versions).
- Recommended practical usage: Add `mirai` as a dev-dependency or use the official MIRAI VSCode extension / custom cargo subcommand setup.
- No baseline warnings could be collected in this environment due to installation limitations.

## Recommended Next Steps
1. Set up MIRAI following official instructions (usually requires building from source or using pre-built binaries from Meta).
2. Run MIRAI on `polarity.rs` focusing on:
   - Range analysis of `prediction_error()`
   - Mutual exclusivity of `positive()` and `negative()`
   - Construction of `DivergenceReason` variants
3. Compare MIRAI results with Kani proofs on the integer-scaled core.
4. Address any warnings before landing the module.

## Notes
MIRAI is best used as a fast development-time check. For CI, a more robust installation method (e.g., pre-built binary or Nix derivation) will be needed.
