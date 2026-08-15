import { latticeSnapshot } from "@/lib/lattice";
import { getFirebaseApp } from "@/lib/firebase";

export default function HomePage() {
  const snap = latticeSnapshot();
  let sdk = "miss";
  try {
    const app = getFirebaseApp();
    sdk = app.options.projectId ?? "init";
  } catch {
    sdk = "local-fallback";
  }

  return (
    <>
      <h1>RESON8 · LOGOS lattice</h1>
      <p className="badge">
        App Hosting projection · firebase project {sdk} · routes registered below
      </p>
      <pre className="etch">
        {snap.routes
          .map((r) => `/${r.id.padEnd(10)}  ${r.tree.padEnd(22)}  ${r.role}`)
          .join("\n")}
      </pre>
      <p className="badge">
        metrics twins: /agda /apps /crates /cutiles /docs /kernels /lean
        /notebooks /ops /tools
      </p>
    </>
  );
}
