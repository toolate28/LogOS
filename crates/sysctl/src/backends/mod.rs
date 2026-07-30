//! Backend implementations for each package manager.
//!
//! Each backend implements [`crate::package::PackageManager`] by shelling
//! out to the appropriate CLI tool and parsing its output.
//!
//! Active backends are selected at runtime based on which tools are
//! available on the current substrate.

pub mod winget;

// Future backends (same trait, different CLI):
// pub mod chocolatey;
// pub mod scoop;
// pub mod apt;
// pub mod nix;
