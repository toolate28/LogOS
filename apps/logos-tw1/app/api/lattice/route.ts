import { NextResponse } from "next/server";
import { latticeSnapshot } from "@/lib/lattice";

export function GET() {
  return NextResponse.json(latticeSnapshot());
}
