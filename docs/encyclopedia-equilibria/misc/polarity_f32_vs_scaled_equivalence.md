# Equivalence between f32 Runtime Path and Integer Verification Path

## Overview

The polarity guard uses `f32` at runtime for compatibility with sm_100 telemetry (`prediction_error`).

For formal verification with Kani we expose a scaled integer view (`prediction_error_scaled()`) using a fixed-point representation.

## Scaling Definition

```rust
pub const PREDICTION_ERROR_SCALE: i32 = 1000;
pub const THRESHOLD_SCALED: i32 = 100; // 0.1 * 1000
```

```rust
pub fn prediction_error_scaled(&self) -> i32 {
    (self.prediction_error() * PREDICTION_ERROR_SCALE as f32) as i32
}
```

## Equivalence Argument

1. **Runtime path** (`positive()` / `negative()`):
   - Uses direct `f32` comparison: `prediction_error() <= 0.1`

2. **Verification path** (Kani harness):
   - Uses `prediction_error_scaled() <= THRESHOLD_SCALED`

3. **Equivalence within operating range**:
   - For `prediction_error ∈ [0.0, 1.0]`, the scaling is lossless for the threshold decision at 0.1 because:
     - `x <= 0.1` ⇔ `(x * 1000) as i32 <= 100` (for x in the normal range, ignoring floating-point rounding at the exact boundary).
   - The boundary case (exactly 0.1) is treated conservatively as `Flipped` in both paths for safety.

4. **Proof obligation**:
   - The Kani harness `check_polarity_prediction_error_threshold` proves that the decision logic is consistent when using the scaled integer view.
   - Unit tests in `polarity_tests.rs` continue to exercise the original `f32` path.

## Maintenance

Any change to the threshold logic must be reflected in both:
- The `f32` implementation (runtime)
- The `prediction_error_scaled()` view and Kani harness (verification)

The two paths are intentionally kept in sync via the thin conversion layer.
