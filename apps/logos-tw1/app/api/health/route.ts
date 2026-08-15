import { NextResponse } from "next/server";
import { getAdminApp } from "@/lib/firebase-admin";

export function GET() {
  let admin = "miss";
  try {
    const app = getAdminApp();
    admin = app.options.projectId ?? "ok";
  } catch (err) {
    admin = err instanceof Error ? err.message : "init-failed";
  }
  return NextResponse.json({
    ok: true,
    backend: "logos-tw1",
    project: process.env.GCLOUD_PROJECT ?? "tri-weavon",
    admin,
    invariant: "alpha+omega=15",
    category: "C",
  });
}
