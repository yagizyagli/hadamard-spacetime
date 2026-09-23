//! Space-time runtime anomaly and precision constraint validation error definitions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpacetimeError {
    /// Velocity parameter matches or transcends local speed of light threshold.
    SuperluminalVelocityViolation,
    
    /// Target coordinates position the vehicle boundary within a gravitational collapse or core radius breach threshold.
    SchwarzschildRadiusBreach,
    
    /// Math parsing overflow or floating point representation instability.
    NumericalPrecisionLoss,
    
    /// Deserialization anomaly during structural telemetry decoding sequences.
    MalformedTelemetryPacket,

    /// Relativistic mass expansion transcends flight computer memory bounds or physical propulsion capability limits.
    RelativisticMassOverflow,

    /// Relativistic phase displacement metrics exceed the quantum entanglement coherence threshold, breaking the QKD link.
    QuantumDecoherenceCrisis,
}

/// Specialized Result type alias for hadamard-spacetime runtime execution contexts.
pub type SpacetimeResult<T> = Result<T, SpacetimeError>;
