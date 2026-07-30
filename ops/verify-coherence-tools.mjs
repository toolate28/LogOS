#!/usr/bin/env node
/**
 * Quick coherence-mcp bedrock tool verification (stdio JSON-RPC).
 * Usage: node ops/verify-coherence-tools.mjs
 */
import { spawn } from "child_process";

const LOGOS_ROOT = process.env.LOGOS_ROOT || "F:/Users/Matthew Ruhnau/LogOS";
const MCP_ROOT = process.env.COHERENCE_MCP_ROOT || "F:/Users/Matthew Ruhnau/coherence-mcp";
const serverCmd = process.env.COHERENCE_MCP_CMD || "node";
const serverArgs = process.env.COHERENCE_MCP_ARGS
  ? process.env.COHERENCE_MCP_ARGS.split(",")
  : [`${MCP_ROOT}/build/index.js`];

const probes = [
  { tool: "invariant_check", args: { alpha: 7, omega: 8 } },
  { tool: "rust_toolchain_status", args: {} },
  { tool: "rust_workspace_status", args: {} },
  { tool: "edge_endpoint_lookup", args: { target: "triweavon", probe: true } },
  { tool: "wave_coherence_check", args: { documentation: "alpha + omega = 15", code: "const sum = 7 + 8;", threshold: 60 } },
];

function spawnServer() {
  return spawn(serverCmd, serverArgs, {
    stdio: ["pipe", "pipe", "inherit"],
    env: {
      ...process.env,
      LOGOS_ROOT,
      RESON8_LOGOS_ROOT: LOGOS_ROOT,
      CTWFI_INVARIANT: "alpha+omega=15",
      CTWFI_STRAND: "reason",
    },
    shell: false,
  });
}

async function main() {
  const child = spawnServer();
  let buf = "";
  const pending = new Map();
  let nextId = 1;

  child.stdout.on("data", (chunk) => {
    buf += chunk.toString();
    const lines = buf.split("\n");
    buf = lines.pop() ?? "";
    for (const line of lines) {
      if (!line.trim()) continue;
      try {
        const msg = JSON.parse(line);
        if (msg.id && pending.has(msg.id)) {
          pending.get(msg.id).resolve(msg);
          pending.delete(msg.id);
        }
      } catch {
        // ignore non-json noise
      }
    }
  });

  const send = (payload) =>
    new Promise((resolve, reject) => {
      const id = nextId++;
      pending.set(id, { resolve, reject });
      child.stdin.write(JSON.stringify({ ...payload, id }) + "\n");
      setTimeout(() => {
        if (pending.has(id)) {
          pending.delete(id);
          reject(new Error(`timeout waiting for ${payload.method}`));
        }
      }, 30000);
    });

  await send({ jsonrpc: "2.0", method: "initialize", params: {
    protocolVersion: "2024-11-05",
    capabilities: {},
    clientInfo: { name: "verify-coherence-tools", version: "0.1.0" },
  }});
  child.stdin.write(JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" }) + "\n");

  const results = [];
  for (const probe of probes) {
    try {
      const res = await send({
        jsonrpc: "2.0",
        method: "tools/call",
        params: { name: probe.tool, arguments: probe.args },
      });
      const text = res.result?.content?.[0]?.text ?? JSON.stringify(res);
      const parsed = (() => { try { return JSON.parse(text); } catch { return text; } })();
      results.push({ tool: probe.tool, ok: !res.error, parsed });
      console.log(`\n=== ${probe.tool} ===`);
      console.log(typeof parsed === "string" ? parsed : JSON.stringify(parsed, null, 2));
    } catch (err) {
      results.push({ tool: probe.tool, ok: false, error: String(err) });
      console.error(`\n=== ${probe.tool} FAILED ===`, err.message);
    }
  }

  child.kill();
  const passed = results.filter((r) => r.ok).length;
  console.log(`\n--- verify-coherence-tools: ${passed}/${results.length} OK ---`);
  process.exit(passed === results.length ? 0 : 1);
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});