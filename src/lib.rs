//! # hadamard-spacetime
//!
//! An enterprise-grade engine calculating real-time relativistic runtime anomalies,
//! localized gravitational time dilations, and quantum phase drift calibrations 
//! tailored for deep-space telemetry mesh architectures and interplanetary missions.

pub mod constants;
pub mod error;
pub mod core;
pub mod telemetry;

#[cfg(feature = "quantum")]
pub mod quantum;

/// Returns the current production deployment string and signature of the engine kernel.
pub fn engine_identity() -> &'static str {
    "hadamard-spacetime-v0.1.0-alpha"
}
