//! reson8-tui — Terminal dashboard (eye of the needle).
//!
//! Binary: `reson8-forge`. Formal diagnostics + LSP live in [`lsp`]; UI in bin.
//! Satellites (`barcode-tui`, `triweave`) are not the needle.

pub mod lsp;

// Historical placeholder modules (not yet extracted from bin):
pub mod dashboard {
    /* Main TUI layout — see bin ui.rs */
}
pub mod evenstar {
    /* Evenstar Resonance widget */
}
pub mod barcodes {
    /* TDA barcode rendering — see crates/barcode-tui */
}
pub mod atom_trail {
    /* ATOM trail log widget */
}
