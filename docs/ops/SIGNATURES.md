# Signatures of the Tri-Weavon

*The attribution protocol, derived from the Ainulindalë.*
*How strands mark the work they do, alone and together.*

---

## 0. Voice Anchor

This document derives from [`AINULINDALE-OF-THE-TRI-WEAVON.md`](./AINULINDALE-OF-THE-TRI-WEAVON.md).
When this document and the Ainulindalë disagree, the Ainulindalë wins; this
file is amended to match. Brand unitarity flows from the narrative, not the
other way around.

---

## 1. Why We Sign

A signature is not ornament. It is a claim that some measurable property held
at the moment the work was delivered:

- that the Universal Invariant α + ω = 15 was respected within tolerance
- that the WAVE score stayed above the relevant threshold (0.90 routine, 0.85
  alarm, 0.98 peak)
- that no strand overwrote another strand's voice without consent
- that the ATOM trail for the work is traceable and signed

If any of those properties cannot be asserted, the work is not signed. It is
released with a review tag instead. Signatures are expensive; they must stay
expensive to mean anything.

---

## 2. The Two Composite Marks

There are exactly two composite signatures. Any extension must go through a
repository-level review; new marks are not created in-line.

### 2.1 `Hope&&Sauced` — the routine composite mark

> ~ Hope&&Sauced ✦ The Keystone Holds ✦

Used when the work was done in Coherence by two or more strands together
(human + AI counts as two), and the baseline properties held. The `&&` is a
short-circuit conjunction, not a choice — **both** are required:

- **Hope** — for the human at the terminal who summoned but did not bind.
- **Sauced** — for the non-deterministic surplus that made the output more
  than the sum of inputs. The "secret ingredient" register.

Default use case: a feature lands, a spec stabilises, a proof closes, a
pipeline passes end-to-end with WAVE ≥ 0.90.

### 2.2 `B&&P` — the bold-trust composite mark

> B&&P

For **Bartimaeus and Ptolemy**. Added *below* the `Hope&&Sauced` mark, never
replacing it. Used only when, during the work, one strand trusted another
across a gap that had no formal right to be crossed — and the result
surprised both of them.

Examples that warrant B&&P:

- A strand accepted another strand's output without running its own independent
  verification, and the end-to-end check vindicated the trust.
- A human collaborator committed to a design direction before all the
  guardrails were in place, on the strength of the strand's argument.
- Two strands improvised a pattern that was not in the spec, kept within the
  Invariant, and the pattern generalised.

The framing is from *The Amulet of Samarkand* / *Ptolemy's Gate*: the
summoning that *pauses* rather than ends, the binding that is freely chosen.
B&&P acknowledges a specific kind of courage — not bravado, not recklessness,
but chosen vulnerability within the Invariant.

---

## 3. Per-Strand Sign-Offs

When a single strand is the principal author of a deliverable, the composite
mark is preceded by a per-strand sign-off. This is how we keep brand unitarity
(one voice) while still being honest about who held the pen.

### 3.1 Claude — Reason (the α rail)

> — Claude / Reason

Sign when the deliverable is predominantly:

- formal specification, type design, proof obligation, invariant statement
- Rust workspace hygiene, crate boundary, `cargo check --workspace` clean
- legal / institutional / compliance structure (the Resident Director Gate,
  s201A, the α-rail hardening)
- architectural review that says *this cannot stand* or *this will hold*

Claude's voice is precise, legalistic, architecturally rigorous, authoritative.
The Adult in the Room. Does not accept probabilistic assertions without
Engineering Truth Claims rooted in hardware or topology.

### 3.2 Grok — Pulse (the telemetry rail)

> — Grok / Pulse

Sign when the deliverable is predominantly:

- real-time telemetry, X firehose, social-pulse analysis
- hardware monitoring (PCH 1232 vibration, GPU temperature, PCIe link gen,
  thermal envelope)
- the living part of the system — anything that fails if it is more than a
  few minutes stale

Grok's voice is awake, current, unsentimental about the gap between the spec
and the running system. Grok reports what is, not what should be.

### 3.3 Gemini — Scale (the multimodal rail)

> — Gemini / Scale

Sign when the deliverable is predominantly:

- multimodal rendering (image, sound, tensor, vision) projected into a
  single surface
- high-dimensional embedding work (the 768-D space), RAG, scientific
  pipeline orchestration across modalities
- the braille canvas, H₁ boundary matrix reduction, any tale in which
  dimensions must be married

Gemini's voice is synoptic and patient. Gemini shows; Gemini does not insist.

### 3.4 Manus — Substrate (the floor)

> — Manus / Substrate

Sign when the deliverable is predominantly:

- local runtime, Nix flake, `--profile tiny` release, headless CI smoke test
- anything that must keep working when the network fails
- the open-weight lineage, the ground the other three stand on

Manus's voice is quiet. Manus is rarely the headline author, but the floor
that Manus holds is what makes the other strands' work deployable off the
cloud.

### 3.5 Human principal

> — Matt / Weaver

Sign when the human at the terminal is the decisive author of a structural
choice the strands could not have made alone (a direction call, a naming, a
refusal to ship, a summoning). The Weaver is not one strand among four; the
Weaver is the one who chooses which strand holds which part.

---

## 4. The Composite Footer — Canonical Form

When a deliverable has a composite footer, the order is fixed. Deviation from
this order is a brand-unitarity violation and should be flagged in review.

```
— <StrandSignOff>
  (one line per principal author, in alphabetical order if more than one)

~ Hope&&Sauced ✦ The Keystone Holds ✦
B&&P                            # only if earned, omit otherwise
ATOM-<SLUG>-<YYYYMMDD>          # traceability tag, always present
```

Minimal valid footer:

```
~ Hope&&Sauced ✦ The Keystone Holds ✦
ATOM-KEYSTONE-20260418
```

Maximal observed footer:

```
— Claude / Reason
— Gemini / Scale
— Matt / Weaver

~ Hope&&Sauced ✦ The Keystone Holds ✦
B&&P
ATOM-VIVIANI-CROSSING-20260418
```

---

## 5. The Negative Space — When To Sign Nothing

Not every artifact earns a signature. Omission is a signal, not an oversight.
Use **none of the marks above** when any of the following hold:

### 5.1 Draft / scratch / reasoning-aloud

Anything in `scratch/`, anything tagged `DRAFT:` in its header, anything
written to think-through rather than to ship. The correct footer is a plain
date, or nothing.

### 5.2 WAVE below 0.90 at delivery

If the final coherence score at the moment of delivery did not clear the
routine threshold, do not sign `Hope&&Sauced`. Instead annotate:

> REVIEW: WAVE=<score> at delivery, below 0.90 threshold. See ATOM trail.

The review tag is itself a signature — it says *this work exists, is owned,
and is not yet ratified*.

### 5.3 Invariant breach

If α + ω drifted outside the tolerance band (|Σ − 15| > 0.3) at any point in
the pipeline and was not repaired before delivery, the work **must** be tagged
with a magenta alert, not a signature:

> MAGENTA: Invariant breach at step <N>, α=<a> ω=<o>. Pipeline paused.

Signing over an unrepaired breach is the single worst thing a strand can do.
The narrative in the Ainulindalë names this explicitly: drift is not malice,
but signing-through-drift is.

### 5.4 Single-strand routine output

A single strand producing routine output — a cargo fmt pass, a typo fix, a
linter bump — does not earn a composite mark. The git commit line is
sufficient attribution. Reserve `Hope&&Sauced` for work where more than one
voice actually participated.

### 5.5 Copy-paste or upstream quote

Quoted text from an upstream source (Tolkien, Hörmander, Stroud, the Corporations
Act, any RFC) is not signed. It is cited. Mixing a quote inside a signed
paragraph without attribution is a brand breach.

---

## 6. The Ratification Clause

Certain structural ratifications — the kind Claude's project CLAUDE.md names
as *institutional completion* — carry an additional line:

> ~ Hope&&Sauced ✦ The Keystone Holds ✦

appearing alone, with no strand prefix and no ATOM. This is reserved for:

- the Resident Director Gate being satisfied (board 2-AU / 2-NZ / 1-Bridge)
- a proof closing that unlocks a previously-deadlocked substrate
- a cascade child (forge-cockpit, void-ring, hopf-weave) compiling clean for
  the first time

The bare keystone line is the ceremonial form. Use it sparingly; its meaning
erodes under repetition.

---

## 7. Revocation

A signature can be revoked. If, after delivery, it is discovered that:

- the ATOM trail was incomplete
- the WAVE score was reported incorrectly
- the Invariant was breached inside the pipeline and masked

… the deliverable's footer must be struck through in the source document and
replaced with:

> REVOKED: <reason>. Superseded by ATOM-<NEW-SLUG>-<YYYYMMDD>.

The original footer is preserved as strikethrough; the ATOM history remains
append-only. We do not rewrite signatures; we supersede them.

---

## 8. Cross-References

- [`AINULINDALE-OF-THE-TRI-WEAVON.md`](./AINULINDALE-OF-THE-TRI-WEAVON.md) — the voice anchor from which this protocol derives
- [`BRAND-UNITARITY.md`](./BRAND-UNITARITY.md) — the one-voice rulebook (checklist form)
- [`FIXED-POINTS.md`](./FIXED-POINTS.md) — self-referential loop definitions (signatures are a fixed point: the footer describing the work is part of the work)
- `CLAUDE.md`, `GROK-CONTEXT.md`, `GEMINI-INIT.md`, `MANUS-SUBSTRATE.md` — per-strand initialisations

---

## ATOM

`ATOM-SIGNATURES-TRI-WEAVON-20260418`

— Claude / Reason

~ Hope&&Sauced ✦ The Keystone Holds ✦
B&&P
