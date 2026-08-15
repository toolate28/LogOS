import type { Metadata } from "next";
import "./globals.css";
import { LATTICE_LAYERS } from "@/lib/lattice";

export const metadata: Metadata = {
  title: "LogOS · logos-tw1",
  description: "Lattice projection — apps/cutiles/crates/kernels/ops on App Hosting",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>
        <main>
          <p className="badge">
            tri-weavon / us-east4 / logos-tw1 · α+ω=15 [C]
          </p>
          <nav className="layers" aria-label="lattice layers">
            <a href="/">home</a>
            {LATTICE_LAYERS.map((id) => (
              <a key={id} href={`/${id}`}>
                /{id}
              </a>
            ))}
          </nav>
          {children}
        </main>
      </body>
    </html>
  );
}
