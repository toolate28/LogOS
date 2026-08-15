import { notFound } from "next/navigation";
import {
  isLatticeLayer,
  LATTICE_LAYERS,
  LAYER_META,
  type LatticeLayer,
} from "@/lib/lattice";

export function generateStaticParams() {
  return LATTICE_LAYERS.map((layer) => ({ layer }));
}

export default async function LayerPage({
  params,
}: {
  params: Promise<{ layer: string }>;
}) {
  const { layer } = await params;
  if (!isLatticeLayer(layer)) notFound();
  const id = layer as LatticeLayer;
  const meta = LAYER_META[id];

  return (
    <>
      <h1>/{id}</h1>
      <pre className="etch">{`${meta.title}
tree   ${meta.tree}
role   ${meta.role}
route  /${id}
`}</pre>
      <p className="badge">
        Presence projection only — not cargo green. logos-activate / TUI [A]
        remain the operator probe.
      </p>
    </>
  );
}
