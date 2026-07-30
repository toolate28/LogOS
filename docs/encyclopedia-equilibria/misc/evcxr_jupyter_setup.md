# evcxr Jupyter Setup — reson8-Labs / TriWeavon

**MeaningSeed**: Hybrid notebook development using Rust (`evcxr`) + Python + Markdown with strong invariant enforcement.

## Quick Start

```bash
nix develop
jupyter lab
```

Select the **Rust** kernel.

## Recommended JupyterLab Extensions (Curated Set)

Add these to your `flake.nix` under the JupyterLab override for full reproducibility:

```nix
jupyterlab = pkgs.python3Packages.jupyterlab.override {
  extraPackages = ps: with ps; [
    jupyterlab-lsp
    jupyterlab-plotly
    jupyterlab-variable-inspector
    jupyterlab-toc
    # Optional but recommended
    jupyterlab-git
    jupyter-matplotlib
  ];
};
```

### Variable Inspector Usage (Recommended Workflow)

**Why use it**:
- Quickly inspect R-matrix outputs and intermediate quantum states from Rust cells.
- See variable shapes and values without scattering print statements.
- Especially useful when developing `fundamental_r_matrix` and `spin32_r_matrix`.

**How to use**:

1. Open the **Variable Inspector** from the right sidebar (or `View → Activate Command Palette → Variable Inspector`).

2. At the top of `Agent.ipynb`, add the following cell template:

```markdown
**Session Invariants**
- `alpha + omega = 15` → **VERIFIED**
- `WAVE coherence ≥ 0.85` → **ENFORCED**

> **Tip**: Keep the Variable Inspector open on the right while working.
> It will show variables from the active Rust (evcxr) kernel in real time.
```

3. Example workflow cell:

```rust
// === R-MATRIX DEVELOPMENT ===
let q = 1.618033988749895_f64;

let r_fundamental = fundamental_r_matrix(q);
// Variable Inspector should now display: q, r_fundamental

let r_spin32 = spin32_r_matrix(q);
// Variable Inspector shows shape and sample elements of the 16×16 matrix
```

**Expected behavior with `spin32_r_matrix`**:
- The inspector displays the variable name, type, and shape (`16×16`).
- For `ndarray` matrices it shows a preview of elements.
- You can expand the view to inspect specific indices.

## Invariant Enforcement

Every notebook must start with explicit invariant assertions (visible in both Markdown and code).

Run before committing:
```bash
coherence-mcp invoke invariant_check
coherence-mcp invoke wave_coherence_check
```

## Known Gotchas

- First Rust cell after kernel start can be slow (incremental compilation).
- Complex custom types may appear opaque in the inspector — fall back to `println!("{:?}", var);`.
- The inspector works per-kernel. Cross-kernel (Rust ↔ Python) inspection requires additional setup.

## Related Files

- `notebooks/Agent.ipynb` — Main R-matrix and quantum walk development
- `notebooks/Platform.ipynb` — System monitoring layer
- Root `flake.nix` — Reproducible environment with extensions

**Argonath Seal**: This workflow is validated for the current TriWeavon checkpoint.
