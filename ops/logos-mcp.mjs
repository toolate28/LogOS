#!/usr/bin/env node
/**
 * LogOS shell ↔ coherence-mcp CLI bridge.
 *
 * Usage:
 *   node ops/logos-mcp.mjs list
 *   node ops/logos-mcp.mjs gauge --alpha 7 --omega 8
 *   node ops/logos-mcp.mjs wave --content "alpha + omega = 15"
 *   node ops/logos-mcp.mjs rust
 *   node ops/logos-mcp.mjs workspace
 *   node ops/logos-mcp.mjs call <tool> [--json '{...}']
 *   node ops/logos-mcp.mjs store --key logos.shell --content "..." --platform grok
 *
 * Env:
 *   LOGOS_ROOT, COHERENCE_MCP_ROOT, COHERENCE_MCP_CMD, COHERENCE_MCP_ARGS
 */
import { spawn } from "child_process";
import { existsSync } from "fs";
import { resolve, join } from "path";

const LOGOS_ROOT =
  process.env.LOGOS_ROOT || "F:/Users/Matthew Ruhnau/LogOS";

function resolveMcpRoot() {
  if (process.env.COHERENCE_MCP_ROOT && existsSync(process.env.COHERENCE_MCP_ROOT)) {
    return process.env.COHERENCE_MCP_ROOT;
  }
  const sibling = resolve(LOGOS_ROOT, "..", "coherence-mcp");
  const nested = join(LOGOS_ROOT, "coherence-mcp");
  if (existsSync(join(sibling, "build", "index.js"))) return sibling;
  if (existsSync(join(nested, "build", "index.js"))) return nested;
  if (existsSync(sibling)) return sibling;
  return nested;
}

const MCP_ROOT = resolveMcpRoot();
const serverCmd = process.env.COHERENCE_MCP_CMD || "node";
const defaultEntry = join(MCP_ROOT, "build", "index.js");
const serverArgs = process.env.COHERENCE_MCP_ARGS
  ? process.env.COHERENCE_MCP_ARGS.split(",")
  : [defaultEntry];

function usage() {
  console.log(`logos-mcp — coherence-mcp tool bridge
  list                         List tools (tools/list)
  gauge [--alpha N] [--omega N]
  wave  --content "..."
  rust                         rust_toolchain_status
  workspace                    rust_workspace_status
  store --key K --content C [--platform grok]
  call  <tool> [--json '{...}']
`);
}

function parseArgs(argv) {
  const out = { _: [] };
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    if (a === "--json" && argv[i + 1]) {
      out.json = argv[++i];
    } else if (a.startsWith("--") && argv[i + 1] && !argv[i + 1].startsWith("--")) {
      out[a.slice(2)] = argv[++i];
    } else if (a.startsWith("--")) {
      out[a.slice(2)] = true;
    } else {
      out._.push(a);
    }
  }
  return out;
}

function spawnServer() {
  if (!existsSync(serverArgs[0]) && serverCmd === "node") {
    throw new Error(
      `coherence-mcp entry missing: ${serverArgs[0]}\nSet COHERENCE_MCP_ROOT or build the MCP server.`
    );
  }
  return spawn(serverCmd, serverArgs, {
    stdio: ["pipe", "pipe", "pipe"],
    env: {
      ...process.env,
      LOGOS_ROOT,
      RESON8_LOGOS_ROOT: LOGOS_ROOT,
      COHERENCE_MCP_ROOT: MCP_ROOT,
      CTWFI_INVARIANT: "alpha+omega=15",
      CTWFI_STRAND: process.env.CTWFI_STRAND || "reason",
    },
    shell: false,
  });
}

async function withClient(fn) {
  const child = spawnServer();
  let buf = "";
  const pending = new Map();
  let nextId = 1;
  let stderr = "";

  child.stderr.on("data", (c) => {
    stderr += c.toString();
  });

  child.stdout.on("data", (chunk) => {
    buf += chunk.toString();
    const lines = buf.split("\n");
    buf = lines.pop() ?? "";
    for (const line of lines) {
      if (!line.trim()) continue;
      try {
        const msg = JSON.parse(line);
        if (msg.id != null && pending.has(msg.id)) {
          pending.get(msg.id).resolve(msg);
          pending.delete(msg.id);
        }
      } catch {
        // ignore non-json
      }
    }
  });

  const send = (payload) =>
    new Promise((resolveP, reject) => {
      const id = nextId++;
      const timer = setTimeout(() => {
        if (pending.has(id)) {
          pending.delete(id);
          reject(new Error(`timeout waiting for ${payload.method}`));
        }
      }, 45000);
      pending.set(id, {
        resolve: (msg) => {
          clearTimeout(timer);
          resolveP(msg);
        },
      });
      child.stdin.write(JSON.stringify({ ...payload, id }) + "\n");
    });

  try {
    await send({
      jsonrpc: "2.0",
      method: "initialize",
      params: {
        protocolVersion: "2024-11-05",
        capabilities: {},
        clientInfo: { name: "logos-mcp", version: "1.0.0" },
      },
    });
    child.stdin.write(
      JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" }) + "\n"
    );
    const result = await fn(send);
    child.kill();
    return result;
  } catch (e) {
    child.kill();
    if (stderr.trim()) e.message += `\nstderr: ${stderr.trim().slice(0, 500)}`;
    throw e;
  }
}

async function callTool(send, name, args = {}) {
  const res = await send({
    jsonrpc: "2.0",
    method: "tools/call",
    params: { name, arguments: args },
  });
  if (res.error) {
    throw new Error(`${name}: ${JSON.stringify(res.error)}`);
  }
  return res.result;
}

function pretty(result) {
  if (!result) {
    console.log("(empty)");
    return;
  }
  if (Array.isArray(result.content)) {
    for (const c of result.content) {
      if (c.type === "text") {
        try {
          const j = JSON.parse(c.text);
          console.log(JSON.stringify(j, null, 2));
        } catch {
          console.log(c.text);
        }
      } else {
        console.log(JSON.stringify(c, null, 2));
      }
    }
    return;
  }
  console.log(JSON.stringify(result, null, 2));
}

async function main() {
  const args = parseArgs(process.argv.slice(2));
  const cmd = args._[0];
  if (!cmd || cmd === "help" || cmd === "-h" || cmd === "--help") {
    usage();
    process.exit(0);
  }

  console.error(`mcp_root=${MCP_ROOT}`);
  console.error(`entry=${serverArgs.join(" ")}`);

  if (cmd === "list") {
    const result = await withClient(async (send) => {
      const res = await send({
        jsonrpc: "2.0",
        method: "tools/list",
        params: {},
      });
      return res.result;
    });
    const tools = result?.tools ?? [];
    for (const t of tools) {
      console.log(`${t.name}\t${(t.description || "").slice(0, 80)}`);
    }
    console.error(`\n${tools.length} tools`);
    return;
  }

  // Bedrock 12-tool server vs extended catalog (mcps/ schemas). rust_* only on extended.
  const toolMap = {
    gauge: () => ({
      name: "gauge_verify",
      args: {
        alpha: Number(args.alpha ?? 7),
        omega: Number(args.omega ?? 8),
      },
    }),
    wave: () => ({
      name: "wave_coherence_check",
      args: {
        content:
          args.content ||
          args.documentation ||
          "LogOS shell command surface α + ω = 15",
      },
    }),
    rust: () => ({ name: "rust_toolchain_status", args: {} }),
    workspace: () => ({ name: "rust_workspace_status", args: {} }),
    platforms: () => ({ name: "list_platforms", args: {} }),
    store: () => ({
      name: "store_context",
      args: {
        key: args.key || "logos.command-surface",
        content: args.content || "LogOS shell wired",
        platform: args.platform || "grok",
        alpha: Number(args.alpha ?? 7),
        omega: Number(args.omega ?? 8),
      },
    }),
  };

  let name;
  let toolArgs;
  if (cmd === "call") {
    name = args._[1];
    if (!name) {
      usage();
      process.exit(1);
    }
    toolArgs = args.json ? JSON.parse(args.json) : {};
  } else if (toolMap[cmd]) {
    const m = toolMap[cmd]();
    name = m.name;
    toolArgs = m.args;
  } else {
    // bare tool name
    name = cmd;
    toolArgs = args.json ? JSON.parse(args.json) : {};
  }

  try {
    const result = await withClient((send) => callTool(send, name, toolArgs));
    pretty(result);
  } catch (e) {
    const msg = e.message || String(e);
    if (/not found|Method not found|-32602/i.test(msg)) {
      console.error(msg);
      console.error(
        `\nTool '${name}' not in this MCP build. Run: node ops/logos-mcp.mjs list`
      );
      console.error(
        "Bedrock (12): gauge_verify wave_coherence_check store_context atom_track list_platforms …"
      );
      console.error(
        "Extended catalog schemas live under LogOS/mcps/coherence-mcp/tools/ (rust_* etc.)."
      );
      console.error("Shell mirror for toolchain: logos-status");
      process.exit(2);
    }
    throw e;
  }
}

main().catch((e) => {
  console.error(e.message || e);
  process.exit(1);
});
