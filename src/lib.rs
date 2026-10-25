//! SigmaOS GTK+ Application Suite
//!
//! Provides GTK+-based desktop applications including:
//! - `sigma-bluetooth`: Bluetooth manager (inspired by Linux Mint Blueman)
//! - `sigma-player`: Media player (inspired by Linux Mint Celluloid/MPV)
//!
//! # Principles
//! - MODULAR KERNEL DESIGN: Each app is a separate binary
//! - SEPARATION OF CONCERNS: Bluetooth ≠ media player
//! - PLUG-IN DRIVER MODEL: Apps register capabilities via traits
//! - COMPOSITION OVER INHERITANCE: Use trait objects, not class hierarchies
//! - GTK+ widget toolkit via safe bindings

#![deny(unsafe_code)]
#![warn(missing_docs)]

pub mod bluetooth;
pub mod player;

/// SigmaOS GTK+ suite version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
