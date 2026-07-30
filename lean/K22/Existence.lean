import Mathlib.Data.Real.Basic
import K22.Tomczak

/-!
# K22 Existence — `ExistenceCertificate` bridge from cutile runtime to Tomczak gate
ATOM: SG-EXISTENCE-CERT-EMITTER-20260709 | harmonic_benefit + srac_corrections
-/

namespace K22.Existence

structure ExistenceCertificate where
  bettiProxyBelowThreshold : Bool
  tomczakPreserved : Bool
  maxErrorBound : Float
  reliable : Bool
  waveScore : Float
  alphaOmegaSum : Float
  coherenceDelta : Float
  atomTrailId : String
  kernelVersion : String
  inputStateHash : Option String
  certificateHash : String
  timestampNs : Nat
  sracCorrections : Nat := 0
  harmonicBenefit : Float := 0
  mehlerReliable : Bool := false
  ottoCdCertificate : Bool := false
  deriving Repr

structure TomczakExistence where
  ctx : TomczakContext
  gate : tomczakLift ctx

def alphaOmegaOk (cert : ExistenceCertificate) : Prop :=
  14.95 < cert.alphaOmegaSum ∧ cert.alphaOmegaSum < 15.05

def contextFromCertificate (cert : ExistenceCertificate) : TomczakContext :=
  { bettiProxy := if cert.bettiProxyBelowThreshold then 0 else 200
  , surge := !cert.tomczakPreserved }

def preservesExistence (cert : ExistenceCertificate) : Prop :=
  cert.bettiProxyBelowThreshold ∧
  cert.tomczakPreserved ∧
  cert.reliable ∧
  alphaOmegaOk cert ∧
  cert.waveScore ≥ 0.85 ∧
  cert.sracCorrections ≤ 1024

theorem certificateImpliesTomczakGate
    (cert : ExistenceCertificate)
    (hb : cert.bettiProxyBelowThreshold = true)
    (ht : cert.tomczakPreserved = true)
    (_hr : cert.reliable = true) :
    tomczakLift (contextFromCertificate cert) := by
  simp [contextFromCertificate, hb, ht, tomczakLift, tomczakPreserved]

def fromExistenceCertificate
    (cert : ExistenceCertificate)
    (_h_wave : alphaOmegaOk cert)
    (hb : cert.bettiProxyBelowThreshold = true)
    (ht : cert.tomczakPreserved = true)
    (hr : cert.reliable = true) :
    TomczakExistence :=
  { ctx := contextFromCertificate cert
  , gate := certificateImpliesTomczakGate cert hb ht hr }

def demoCertificate : ExistenceCertificate :=
  { bettiProxyBelowThreshold := true
  , tomczakPreserved := true
  , maxErrorBound := 1e-12
  , reliable := true
  , waveScore := 0.998
  , alphaOmegaSum := 15.0
  , coherenceDelta := 0.001
  , atomTrailId := "ATOM-DEMO-20260709-001"
  , kernelVersion := "cutile-demo-mehler-levin"
  , inputStateHash := some "demo-input-hash"
  , certificateHash := ""
  , timestampNs := 0
  , sracCorrections := 0
  , harmonicBenefit := 0.0
  , mehlerReliable := true
  , ottoCdCertificate := true }

theorem demo_alpha_omega_ok : alphaOmegaOk demoCertificate := by
  unfold alphaOmegaOk demoCertificate
  native_decide

example : TomczakExistence :=
  fromExistenceCertificate demoCertificate demo_alpha_omega_ok rfl rfl rfl

end K22.Existence