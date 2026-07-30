import Lake
open Lake DSL

require mathlib from git
  "https://github.com/leanprover-community/mathlib4" @ "v4.8.0"

package «ns-entropy» where
  leanOptions := #[
    ⟨`autoImplicit, false⟩
  ]

@[default_target]
lean_lib Ns where
  srcDir := "Ns"

lean_lib TriWeavon where
  srcDir := "TriWeavon"

lean_lib K22 where
  globs := #[`K22.+]
