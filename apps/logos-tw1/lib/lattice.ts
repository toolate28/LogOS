/** Lattice layers registered for App Hosting metrics + Next routes. */

export const LATTICE_LAYERS = [
  "agda",
  "apps",
  "crates",
  "cutiles",
  "docs",
  "kernels",
  "lean",
  "notebooks",
  "ops",
  "tools",
] as const;

export type LatticeLayer = (typeof LATTICE_LAYERS)[number];

export const LAYER_META: Record<
  LatticeLayer,
  { title: string; tree: string; role: string }
> = {
  agda: {
    title: "Agda",
    tree: "agda/",
    role: "TriWeavon typecheck · Cubical vendor optional",
  },
  apps: {
    title: "Apps",
    tree: "apps/",
    role: "triweave · mc-bridge · nexus-pulse · logos-tw1",
  },
  crates: {
    title: "Crates",
    tree: "crates/",
    role: "core · tui · sphinx · spiral-safe · wave",
  },
  cutiles: {
    title: "Cutiles",
    tree: "cutiles/cutile",
    role: "claim_gate · DriftGuard · R-matrix host",
  },
  docs: {
    title: "Docs",
    tree: "docs/",
    role: "canon · choke · formal · architecture",
  },
  kernels: {
    title: "Kernels",
    tree: "kernels/",
    role: "blackwell-* · fundamental_r_matrix",
  },
  lean: {
    title: "Lean 4",
    tree: "lean/",
    role: "TriWeavon QR · LatticeLayers · Conservation · K22",
  },
  notebooks: {
    title: "Notebooks",
    tree: "notebooks/",
    role: "backend probes · AUKUS · R-matrix",
  },
  ops: {
    title: "Ops",
    tree: "ops/",
    role: "shell · command-surface · marks · entangle",
  },
  tools: {
    title: "Tools",
    tree: "tools/",
    role: "claim_lint · gait_mono",
  },
};

export function isLatticeLayer(s: string): s is LatticeLayer {
  return (LATTICE_LAYERS as readonly string[]).includes(s);
}

export function latticeSnapshot() {
  return {
    project: "tri-weavon",
    backend: "logos-tw1",
    region: "us-east4",
    invariant: "alpha+omega=15",
    category: "C",
    routes: LATTICE_LAYERS.map((id) => ({
      id,
      path: `/${id}`,
      ...LAYER_META[id],
    })),
  };
}
