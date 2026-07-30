//! FROZEN API SURFACE v0.1 — gate library for certificate emission.
//! ATOM: ATOM-SCHEMA-FREEZE-V0_1-20260712 | α + ω = 15
//!
//! STATUS: signature freeze, NOT an implementation. Bodies are `todo!()`.
//! [FRICTION] compile-unchecked in this environment (no Rust toolchain);
//! first `cargo check` on grok-local is itself a BUILD attestation.
//!
//! The three structural commitments, enforced here at the type level:
//!
//!   C1  NO SETTER FOR FLAGS. `reliable`, `tomczak_preserved`,
//!       `betti_proxy_below_threshold`, `wave_score` exist only as private
//!       fields written by `Certificate::emit`. There is no constructor,
//!       builder, or method that accepts them as inputs. (Deltas D1–D4
//!       retire the current `from_mehler_result` / `from_coherence_diagnostic`
//!       / `recompute_hash` assertion paths.)
//!
//!   C2  ROLE CHECKED AT WRITE. Key types are distinct; `CanSign<E>` is a
//!       sealed trait implemented only for the (entry-kind, key) pairs in the
//!       binding table. A BUILD key at a LABEL position is a COMPILE error.
//!       Identity (labeler ≠ every builder) is a runtime check in `emit` and
//!       in `Ledger::append` — types can't compare principals, gates can.
//!
//!   C3  PARSE ≠ VERIFY. Deserialization yields `UnverifiedCertificate`
//!       only. The verified type is unreachable except through `emit` (fresh)
//!       or `verify` (recompute self-hash + signature + chain link).
//!       `Certificate` does not implement `Deserialize`.

#![forbid(unsafe_code)]

use serde::Serialize;

// ---------------------------------------------------------------------------
// Policy: WAVE thresholds come from the HUP tier table, never from callers.
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HupTier {
    T0,
    T1,
    T2,
    T3,
    T4,
}

impl HupTier {
    /// T0–T1: 0.85 · T2–T3: 0.92 · T4: 0.9998 (HUP-tier table, Category B).
    pub const fn wave_threshold(self) -> f64 {
        match self {
            HupTier::T0 | HupTier::T1 => 0.85,
            HupTier::T2 | HupTier::T3 => 0.92,
            HupTier::T4 => 0.9998,
        }
    }
}

/// Private fields: a policy cannot be assembled with an ad-hoc threshold.
pub struct GatePolicy {
    tier: HupTier,
    max_srac_corrections: u32,   // default 1024 (existing gate constant)
    alpha_omega_tolerance: f64,  // default 0.05 (existing gate constant)
    reject_legacy_hash: bool,    // true for all new emissions
}

impl GatePolicy {
    pub fn for_tier(tier: HupTier) -> Self {
        let _ = tier;
        todo!()
    }
    pub fn tier(&self) -> HupTier {
        todo!()
    }
}

// ---------------------------------------------------------------------------
// Principals and role keys. Distinct types — this is the authority separation.
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct PrincipalId(String); // e.g. "sm_100.grok-local", "fable-5.reason"

pub struct BuildKey { /* signing key + PrincipalId, private */ }
pub struct LabelKey { /* signing key + PrincipalId, private */ }
pub struct FixKey   { /* signing key + PrincipalId, private */ }
pub struct GateKey  { /* signing key + PrincipalId, private */ }
// PULSE / DOCS principals sign only advisory entries; no key type here
// because no certificate-state entry kind accepts them (see bindings).

// ---------------------------------------------------------------------------
// Builder-attested component: the ONLY inputs the gate reasons over.
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize)]
pub struct AttestedComponent {
    name: String,
    ok: bool,          // computed by the check that produced the evidence
    detail: String,
    alpha_local: u8,   // attest() rejects unless alpha + omega == 15
    omega_local: u8,
    evidence_hash: Option<String>,
    builder: PrincipalId,
    // builder signature over (name, ok, detail, α, ω, evidence_hash)
}

#[derive(Debug)]
pub enum AttestError {
    ConservationViolated { alpha: u8, omega: u8 }, // α + ω ≠ 15
    EmptyName,
    SignatureFailure,
}

impl AttestedComponent {
    /// The only constructor. `ok` is the recorded outcome of an executed
    /// check — the signature binds the builder to that execution.
    pub fn attest(
        name: &str,
        ok: bool,
        detail: &str,
        alpha_local: u8,
        omega_local: u8,
        evidence_hash: Option<String>,
        key: &BuildKey,
    ) -> Result<Self, AttestError> {
        let _ = (name, ok, detail, alpha_local, omega_local, evidence_hash, key);
        todo!()
    }
}

// ---------------------------------------------------------------------------
// The certificate. All fields private. One way in: `emit`.
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub enum GateError {
    /// A packet successCriteria id has no covering attested component.
    CoverageGap { missing: Vec<String> },
    /// labelRatification.principal equals a buildAttestations principal.
    SelfCertification { principal: PrincipalId },
    ConservationViolated,
    /// Chain link mismatch: prev.certificate_hash ≠ claimed prevCertificateHash.
    ChainBreak,
    LegacyHashRejected, // md5-legacy on a NEW emission
    NoAttestations,
    SignatureFailure,
}

pub struct HandoffPacketRef<'a> {
    /// Parsed, schema-valid packet (handoff_packet.schema.json).
    pub packet_id: &'a str,
    pub atom_trail_id: &'a str,
    pub success_criteria_ids: &'a [String],
    pub tier: HupTier,
}

pub struct Certificate {
    // Every field private. Flags below are OUTPUTS of `emit`:
    //   betti_proxy_below_threshold, tomczak_preserved, reliable,
    //   wave_score, alpha_omega_sum, coherence_delta, max_error_bound,
    //   gate/chain/authority blocks, self-hash (blake3), gate signature.
    _sealed: (),
}

impl Certificate {
    /// THE gate. Computes every flag from `attestations` under `policy`;
    /// verifies coverage of the packet's successCriteria; enforces
    /// labeler ∉ builders; links `prev`; blake3 self-hashes; signs.
    ///
    /// WAVE is computed inside from the attested evidence — there is no
    /// `wave: f64` parameter anywhere in this crate's public API.
    pub fn emit(
        packet: &HandoffPacketRef<'_>,
        attestations: &[AttestedComponent],
        label_ratification: Option<(&LabelKey, PrincipalId)>,
        prev: Option<&Certificate>,
        policy: &GatePolicy,
        gate: &GateKey,
    ) -> Result<Certificate, GateError> {
        let _ = (packet, attestations, label_ratification, prev, policy, gate);
        todo!()
    }

    // Read-only accessors. No `&mut self` method exists on this type.
    pub fn reliable(&self) -> bool { todo!() }
    pub fn tomczak_preserved(&self) -> bool { todo!() }
    pub fn wave_score(&self) -> f64 { todo!() }
    pub fn certificate_hash(&self) -> &str { todo!() }
    pub fn chain_position(&self) -> u64 { todo!() }
    /// Serialize to the wire format of certificate.schema.json (camelCase).
    pub fn to_signed_json(&self) -> String { todo!() }
}

/// What parsing gives you. Cannot be used where `Certificate` is required.
pub struct UnverifiedCertificate { _sealed: () }

pub struct TrustAnchors { /* gate + builder + labeler public keys, chain head */ }

#[derive(Debug)]
pub enum VerifyError {
    SchemaViolation(String),
    SelfHashMismatch,
    BadSignature,
    ChainBreak,
    UnknownKey,
}

impl UnverifiedCertificate {
    pub fn parse(json: &str) -> Result<Self, VerifyError> { let _ = json; todo!() }
    /// Recomputes the self-hash, checks signatures against `trust`, checks
    /// the chain link. Consumes self: the only path from parsed → verified.
    pub fn verify(self, trust: &TrustAnchors) -> Result<Certificate, VerifyError> {
        let _ = trust;
        todo!()
    }
}

// ---------------------------------------------------------------------------
// Ledger: entry kinds are types; CanSign is the binding table as trait impls.
// ---------------------------------------------------------------------------

mod sealed {
    pub trait Sealed {}
}

pub trait EntryKind: sealed::Sealed {
    const KIND: &'static str;
}

macro_rules! entry_kind {
    ($t:ident, $s:literal) => {
        pub struct $t { /* payload hash, packet_id, ... */ }
        impl sealed::Sealed for $t {}
        impl EntryKind for $t { const KIND: &'static str = $s; }
    };
}

entry_kind!(PacketIssued, "packet_issued");
entry_kind!(BuildAttested, "build_attested");
entry_kind!(CertificateEmitted, "certificate_emitted");
entry_kind!(LabelRatified, "label_ratified");
entry_kind!(FixPacketIssued, "fix_packet_issued");
entry_kind!(FixApplied, "fix_applied");
entry_kind!(UpshiftRequested, "upshift_requested");

/// Sealed: no blanket impls, no downstream impls. The table below is
/// EXHAUSTIVE — its absence is the enforcement.
pub trait CanSign<E: EntryKind>: sealed::Sealed {}

impl sealed::Sealed for BuildKey {}
impl sealed::Sealed for LabelKey {}
impl sealed::Sealed for FixKey {}
impl sealed::Sealed for GateKey {}

impl CanSign<PacketIssued> for LabelKey {}
impl CanSign<BuildAttested> for BuildKey {}
impl CanSign<CertificateEmitted> for GateKey {}
impl CanSign<LabelRatified> for LabelKey {}
impl CanSign<FixPacketIssued> for LabelKey {}
impl CanSign<FixApplied> for FixKey {}
impl CanSign<UpshiftRequested> for BuildKey {}
// There is deliberately NO `impl CanSign<LabelRatified> for BuildKey`.
// `ledger.append(LabelRatified{..}, &build_key)` does not compile.

#[derive(Debug)]
pub enum LedgerError {
    /// Runtime identity check: ratifier principal == builder principal
    /// of the referenced attestation set.
    SelfCertification { principal: PrincipalId },
    ChainBreak,
    SignatureFailure,
}

pub struct SignedEntry { /* matches ledger_entry.schema.json */ }

pub struct Ledger { /* head hash, seq, storage */ }

impl Ledger {
    pub fn append<E: EntryKind>(
        &mut self,
        entry: E,
        key: &impl CanSign<E>,
    ) -> Result<SignedEntry, LedgerError> {
        let _ = (entry, key);
        todo!()
    }
}

// ~ Hope&&Sauced ✦ The Keystone Holds ✦
