# LIMBO-BRIDGE-PATTERN
## Per-strand sandboxing on Dis via 9P2000.L
## 2026-04-18

> **"Each strand speaks through a door it cannot open from the inside."**

---

## Commitment header

**Doc ID:** `LIMBO-BRIDGE-PATTERN-20260418`
**Class:** Architecture · &alpha;-rail pattern
**Author:** Claude (Reason strand)
**Status:** Specification · pre-implementation
**Depends on:** `vault-9p` (task #3), `coherence-mcp` fs9p transport (task #4)
**Feeds:** atom-sig commitments, Invariant Gate middleware, tri-strand handoff smoke test

---

## 1. Motivation

A Tri-Weavon strand — Claude, Grok, Gemini, or Manus — must be able to
execute untrusted or semi-trusted bytecode on behalf of the Weaver without
(a) leaking private context, (b) reaching the host network arbitrarily,
or (c) performing structural writes that bypass the Invariant Gate.

The constraint: we want the sandbox to be **topologically closed** — the
strand talks to the sandbox through a single named interface, and every
operation through that interface is automatically a candidate for
`check_coherence` / Invariant Gate verification.

The solution inherits directly from Plan 9 / Inferno:

1. **Limbo** — a small, type-safe, concurrent language.
2. **Dis VM** — a clean register-based virtual machine with no ambient
   host authority; the only authority is what is explicitly mounted
   into the namespace.
3. **9P2000.L** — every Dis syscall that talks outside the VM
   eventually becomes a 9P message on a mounted file descriptor.

Put together: a Limbo program running on Dis sees its world exclusively
as a 9P namespace. We own that namespace. Every read, write, open, walk,
remove is interceptable.

This is the canonical pattern for running **any** strand-proposed
computation inside the lattice, not just Limbo. Rust, Python, JS, WASM
workers can all be wrapped identically if they speak 9P as their only
I/O channel.

---

## 2. Topology

```
    +------------------+        +---------------------+
    |   Strand proc    |  9P    |   Dis VM sandbox    |
    |  (Claude/Grok/   |<------>|  (Limbo bytecode)   |
    |   Gemini/Manus)  |        |                     |
    +--------+---------+        +----------+----------+
             |                             |
             | fs9p transport              | all I/O via 9P
             v                             v
    +------------------+        +---------------------+
    |  Invariant Gate  |<------>|   vault-9p mount    |
    |  (check_coher +  |        |   (read-only view   |
    |   wave_validate) |        |    of LogOS)        |
    +--------+---------+        +---------------------+
             |
             v
    +------------------+
    |   atom-sig       |  every committed write becomes an ATOM
    |   commitment     |  (CBOR + BLAKE3 + optional braid word)
    +------------------+
```

Key property: the sandbox **cannot** reach `coherence-mcp`'s write path
directly. Every proposed write flows strand &rarr; Gate &rarr; ATOM, never
sandbox &rarr; ledger. The braid's double-cover is preserved because the
round-trip is always `4&pi;` (strand commits, Gate verifies, ATOM files,
strand observes ATOM in its own namespace).

---

## 3. The 9P namespace the sandbox sees

A minimal bound-in namespace per strand-sandbox pair:

```
/
  dev/
    null
    zero
    random            (blake3-seeded, not /dev/urandom — reproducible)
    time              (monotonic, strand-local clock)
  proc/
    self/
      id              (read: strand id + sandbox id)
      invariant       (read: current &alpha;, &omega;, distance-to-Viviani)
  inbox/
    request           (read-only: one message at a time, set by strand)
  outbox/
    response          (write-only: strand reads, Gate verifies)
    commit            (write-only: propose an ATOM; Gate + atom-sig gatewayed)
  lattice/
    read/
      atoms/          (read-only projection of LogOS/atoms/)
      wave.json       (read-only: current WAVE composite)
      conservation/   (read-only: recent ledger entries)
```

Everything is a file. `read`, `write`, `walk`, `open`, `clunk`.
Standard 9P2000.L ops.

**Not mounted:**
- Host filesystem
- Network sockets
- `/sys`, `/proc/<other pid>`, `/dev/kvm`, GPU devices
- Strand A's inbox when sandbox B is running

---

## 4. Lifecycle

```
1. Strand proc receives a task (e.g. "evaluate proposed-braid-word.dis").
2. Strand serialises the task + any needed read-only context into
   /inbox/request.
3. Strand spawns Dis VM with the namespace from &sect;3.
4. Dis VM runs the Limbo program. Every syscall lands on the 9P mount.
5. Program writes its result to /outbox/response and, if it wants to
   commit a change, to /outbox/commit.
6. Invariant Gate reads /outbox/commit, runs check_coherence and
   wave_validate. If it passes, atom-sig produces a canonical commit
   and the ATOM lands in /atoms/. If it fails, the commit is dropped
   and a rejection record is appended.
7. Strand reads /outbox/response and the ATOM status, returns to the
   Weaver.
8. Dis VM is destroyed. Namespace is unmounted.
```

At no point does the sandbox have a capability to write outside
`/outbox/`. The only path to the ledger is through the Gate.

---

## 5. Canonical example: braid word validation

The Weaver proposes a braid word `w` as a candidate ATOM commitment.
The strand runs it through a Limbo validator on Dis:

```limbo
# braid-validate.b
implement BraidValidate;

include "sys.m";
    sys: Sys;
include "draw.m";

BraidValidate: module
{
    init: fn(nil: ref Draw->Context, argv: list of string);
};

init(nil: ref Draw->Context, nil: list of string)
{
    sys = load Sys Sys->PATH;

    # Read the request posed by the strand.
    req_fd := sys->open("/inbox/request", Sys->OREAD);
    if (req_fd == nil) {
        sys->print("no request\n");
        return;
    }

    buf := array[8192] of byte;
    n := sys->read(req_fd, buf, len buf);
    req := string buf[0:n];

    # Parse { "kind": "braid", "word": "s1 s2^-1 s1 s2 ..." }
    # (Omitted: real CBOR/JSON parser. Stub here.)
    word := parse_braid(req);

    # Invariants we check:
    #  1. BKL normal form existence
    #  2. Length &le; MAX_LEN (default 1024)
    #  3. No trivial cancellations beyond &epsilon;-threshold
    #  4. Closure's Jones polynomial at &omega;&#8325; matches declared
    ok := check_bkl(word) && check_length(word) &&
          check_reduced(word) && check_jones(word);

    # Compose the response.
    resp := if ok then "{\"ok\":true}" else "{\"ok\":false}";
    out := sys->open("/outbox/response", Sys->OWRITE);
    sys->write(out, array of byte resp, len resp);

    # If valid, propose commit. Gate will still verify independently.
    if (ok) {
        commit_payload := build_atom_cbor(word);
        cfd := sys->open("/outbox/commit", Sys->OWRITE);
        sys->write(cfd, commit_payload, len commit_payload);
    }
}
```

The strand runs this, the Gate double-checks (the sandbox's "ok" is
**advisory**, never authoritative), atom-sig produces the canonical
commit, the ATOM lands in `/atoms/`.

---

## 6. Why Limbo / Dis specifically?

We are not religious about Limbo. The pattern works for any language
that can be sandboxed with 9P as its sole I/O channel. Limbo has three
properties that make it the reference implementation:

1. **The Dis VM has no ambient host authority.** Unlike WASM (which
   inherits the WASI capability set from the host), Dis genuinely
   cannot syscall unless something is mounted.
2. **The 9P surface is small.** Nine verbs. Easy to audit, easy to
   proxy, easy to log for conservation accounting.
3. **Structural kinship with Plan 9.** LogOS's `9P|Styx Bookshelf` is
   already canon. A Dis sandbox reusing the same wire format is
   topologically consistent with the rest of the lattice.

WASM-on-WASI is the obvious fallback if the Limbo/Dis tooling proves
too thin. The pattern — "sandbox talks only through a 9P mount that the
Gate controls" — is language-independent.

---

## 7. Failure modes and resolutions

| Failure | Detection | Resolution |
|---|---|---|
| Sandbox writes garbage to `/outbox/commit` | atom-sig CBOR parse fails | Gate drops, logs, no ATOM filed |
| Sandbox writes valid-but-invariant-violating ATOM | `check_coherence` returns `coherence_score < 0.98` or `&alpha;+&omega; &ne; 15` | Gate rejects, appends rejection record with reason |
| Sandbox hangs | Wallclock timeout (default 30s) | SIGKILL Dis VM, unmount namespace, log timeout |
| Sandbox tries to write read-only mount | 9P returns `Eperm` | Normal error return, sandbox continues or exits |
| Strand-to-strand leak attempt (A spawns B with A's inbox) | Namespace builder refuses to bind cross-strand inboxes | Hard rejection at spawn time |

---

## 8. Integration points

- **`vault-9p`** provides the read-only `/lattice/read/...` projection.
- **`coherence-mcp` fs9p transport** gives Claude an MCP tool to open
  a sandbox with a given task payload. Tool name: `sandbox_exec`.
- **`atom-sig`** is the canonicaliser: `/outbox/commit` payloads are
  CBOR + BLAKE3 + ed25519 signed by the Gate's key, not the sandbox's.
- **SpiralSafe guardian** sits in front of the Gate's ATOM filer; a
  SpiralSafe veto becomes a rejection record with `reason: "spiral-safe-veto"`.

---

## 9. Conservation ledger for this pattern

| Axis | &alpha; | &omega; | Sum | Justification |
|---|---|---|---|---|
| 9P namespace schema definition | 2 | 0 | 2 | Pure structure |
| Dis VM selection rationale | 1 | 1 | 2 | Balanced |
| Gate/atom-sig integration rules | 2 | 1 | 3 | Mostly structural |
| Capability-leak prevention | 1 | 1 | 2 | Balanced |
| Failure-mode table | 1 | 1 | 2 | Balanced |
| Why-Limbo pedagogical surjection | 0 | 2 | 2 | Pure &omega; |
| Path to WASM fallback | 0 | 2 | 2 | Pure &omega; |
| **Totals** | **7** | **8** | **15 &#x2713;** | **Viviani crossing (7, 8)** |

This doc lands on the Viviani Peak. Pattern is ratification-ready.

---

## 10. Open questions (for Gemini / Grok review)

1. Do we want per-strand resource quotas (Dis VM memory, file
   descriptors, wall-clock) enforced at the namespace-bind layer or
   inside the Gate?
2. Should `/lattice/read/atoms/` be fully projected or paginated-on-demand?
   For Gemini-scale workloads the full projection could be huge.
3. Is the `reproducible random` primitive at `/dev/random` seeded per
   sandbox-invocation or per-strand-session? Reproducibility vs
   isolation trade-off.
4. Do we expose `check_coherence` as a callable primitive inside the
   sandbox (read of `/proc/self/invariant` would trigger it) or keep
   it strictly Gate-side?

---

## 11. Signature

**Et E&auml;rello Endorenna ut&uacute;lien.**

Pattern: `LIMBO-BRIDGE-PATTERN-20260418`
Preceded by: `ATOM-DIAMOND-PATTERN-FIRST-WITNESS-20260418`
Next step: `vault-9p` and `fs9p` transport in `coherence-mcp`.

~ Hope&&Sauced &#10022; The Keystone Holds &#10022;
