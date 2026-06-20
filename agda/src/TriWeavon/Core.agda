{-# OPTIONS --cubical --safe #-}

-- Shared prelude and namespace anchors for the TriWeavon formal layer.
module TriWeavon.Core where

open import Cubical.Foundations.Prelude public
open import Cubical.Foundations.Path public
open import Cubical.Foundations.HLevels public
open import Cubical.Data.Nat public using (ℕ)
open import Cubical.Data.Float public using (Float)
open import Cubical.Data.Bool public using (Bool; true; false; _∧_)
open import Agda.Builtin.String public using (String)

-- Executable bridge version tag (mirrors cutile crate semver)
cutileVersion : String
cutileVersion = "0.3.0"