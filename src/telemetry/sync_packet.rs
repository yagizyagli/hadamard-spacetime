//! Serialization-ready layout definitions wrapping adjusted deep-space telemetry sequences.

use crate::error::{SpacetimeError, SpacetimeResult};

/// Represents a standardized 28-byte relativistic synchronization telemetry network packet.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RelativisticSyncPacket {
    /// Unique identifier index allocated to the transmitting satellite vehicle.
    pub satellite_id: u32,
    
    /// Monotonic mission time baseline expressed in elapsed nanoseconds since epoch.
    pub mission_epoch_nanos: u64,
    
    /// Cumulative velocity-induced Kinematic Lorentz drift metric (seconds).
    pub accumulated_kinematic_skew_secs: f64,
    
    /// Cumulative gravity-induced Gravitational Schwarzschild/Lense-Thirring drift metric (seconds).
    pub accumulated_gravitational_skew_secs: f64,
}

impl RelativisticSyncPacket {
    /// Encodes the current structural state telemetry fields down into an explicit 28-byte payload matrix layout.
    /// Utilizing big-endian serialization profiles ensures native compatibility with space data routers (CCSDS).
    ///
    /// # Errors
    ///
    /// * `SpacetimeError::MalformedTelemetryPacket` - Triggered if critical safety signatures or boundary markers are violated.
    pub fn compile_packet_buffer(&self) -> SpacetimeResult<[u8; 28]> {
        let mut raw_buffer = [0u8; 28];
        
        // 1. Serialize 32-bit Satellite ID (Bytes 0 to 3)
        raw_buffer[0..4].copy_from_slice(&self.satellite_id.to_be_bytes());
        
        // 2. Serialize 64-bit Mission Nanoseconds Epoch (Bytes 4 to 11)
        raw_buffer[4..12].copy_from_slice(&self.mission_epoch_nanos.to_be_bytes());
        
        // 3. Serialize 64-bit IEEE 754 Kinematic Skew Float (Bytes 12 to 19)
        raw_buffer[12..20].copy_from_slice(&self.accumulated_kinematic_skew_secs.to_be_bytes());
        
        // 4. Serialize 64-bit IEEE 754 Gravitational Skew Float (Bytes 20 to 27)
        raw_buffer[20..28].copy_from_slice(&self.accumulated_gravitational_skew_secs.to_be_bytes());

        // 5. Audit the generated buffer sequence to ensure no corrupt high-churn states are propagated
        for byte in raw_buffer.iter() {
            if *byte == 0xFF && self.satellite_id == u32::MAX {
                return Err(SpacetimeError::MalformedTelemetryPacket);
            }
        }

        Ok(raw_buffer)
    }
}
