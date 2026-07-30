import Lean

/-!
# K22 ExprMatch — reusable goal classifiers (string-safe for Lean 4.8)

Uses Name.mkStr* so constants need not be in scope at parse time.
Substring helper replaces String.containsSubstr (Batteries / newer Lean).

ATOM: ATOM-K22-EXPR-MATCH-20260709 | α + ω = 15
-/

namespace K22.ExprMatch

open Lean Meta

/-- Substring test without Batteries. -/
def hasSubstr (s needle : String) : Bool :=
  (s.splitOn needle).length > 1

def isEqType (e : Expr) : Bool :=
  e.isAppOf ``Eq

def hasConst (e : Expr) (n : Name) : Bool :=
  (e.find? fun sub => sub.isConstOf n).isSome

def mentionsK22Decl (e : Expr) (names : Array Name) : Bool :=
  names.any (hasConst e)

def nBurau : Array Name := #[
  Name.mkStr2 "K22" "exponent_sum",
  Name.mkStr2 "K22" "det_burau",
  Name.mkStr2 "K22" "det",
  Name.mkStr2 "K22" "burau_σ₁",
  Name.mkStr2 "K22" "burau_σ₂",
  Name.mkStr2 "K22" "Mat2",
  Name.mkStr1 "Mat2",
  Name.mkStr2 "K22" "jonesPolynomial",
  Name.mkStr2 "K22" "reducedBurauDet",
  Name.mkStr2 "K22" "writheExponent"
]

def isBurauDetExponentGoal (goalType : Expr) (goalStr : String) : Bool :=
  isEqType goalType
    || hasSubstr goalStr "burau"
    || hasSubstr goalStr "Mat2"
    || hasSubstr goalStr "exponent_sum"
    || hasSubstr goalStr "det_burau"
    || mentionsK22Decl goalType nBurau

def mentionsTomczakLift (goalType : Expr) (goalStr : String) : Bool :=
  goalType.isAppOf (Name.mkStr2 "K22" "tomczakLift")
    || goalType.isAppOf (Name.mkStr2 "K22" "tomczakPreserved")
    || hasSubstr goalStr "tomczak"
    || hasSubstr goalStr "tomczak_lift"
    || hasSubstr goalStr "tomczak_preserved"

def mentionsSerrePage (goalType : Expr) (goalStr : String) : Bool :=
  goalType.isAppOf (Name.mkStr2 "K22" "SerreScar")
    || hasSubstr goalStr "SerreScar"
    || hasSubstr goalStr "sracCascade"
    || hasSubstr goalStr "Serre-Scarr"
    || hasSubstr goalStr "dᵣ"
    || hasSubstr goalStr "d_r"

def isJonesPolynomialGoal (goalType : Expr) (goalStr : String) : Bool :=
  hasSubstr goalStr "jonesPolynomial"
    || hasSubstr goalStr "reducedBurauDet"
    || hasSubstr goalStr "writheExponent"
    || mentionsK22Decl goalType #[
        Name.mkStr2 "K22" "jonesPolynomial",
        Name.mkStr2 "K22" "reducedBurauDet"]

def mentionsExistence (_goalType : Expr) (goalStr : String) : Bool :=
  hasSubstr goalStr "ExistenceCertificate"
    || hasSubstr goalStr "preservesExistence"
    || hasSubstr goalStr "TomczakExistence"
    || hasSubstr goalStr "alphaOmegaOk"

def mentionsMOG (_goalType : Expr) (goalStr : String) : Bool :=
  hasSubstr goalStr "isMOGOctad"
    || hasSubstr goalStr "MiracleOctad"
    || hasSubstr goalStr "hexacode"
    || hasSubstr goalStr "Octad"
    || hasSubstr goalStr "MOG"
    || hasSubstr goalStr "applyGenerator"

def classifyK22Goal (goalType : Expr) (goalStr : String) : String :=
  if isJonesPolynomialGoal goalType goalStr then "jones"
  else if isBurauDetExponentGoal goalType goalStr then "burau"
  else if mentionsMOG goalType goalStr then "mog"
  else if mentionsExistence goalType goalStr then "existence"
  else if mentionsTomczakLift goalType goalStr then "tomczak"
  else if mentionsSerrePage goalType goalStr then "serre"
  else "other"

def shouldUseRingMatrix (goalType : Expr) (goalStr : String) : Bool :=
  isBurauDetExponentGoal goalType goalStr
    || mentionsTomczakLift goalType goalStr

def classifyGoalType (goalType : Expr) : MetaM String := do
  let goalStr := (← ppExpr goalType).pretty
  pure (classifyK22Goal goalType goalStr)

end K22.ExprMatch
