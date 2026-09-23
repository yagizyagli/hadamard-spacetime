//! Special Relativistic kinematic dilation calculus utilizing native Lorentz conversions.

use crate::constants::SPEED_OF_LIGHT;
use crate::error::{SpacetimeError, SpacetimeResult};

/// Calculates the proper time interval duration error observed under specific velocity metrics.
/// Returns the accumulated time skew in seconds based on Special Relativity.
///
/// # Arguments
///
/// * `velocity_vectors_ms` - Instantaneous velocity vector scalar relative to the observer (meters / second).
/// * `delta_time_seconds` - Ground baseline delta observation window span (seconds).
///
/// # Errors
///
/// * `SpacetimeError::SuperluminalVelocityViolation` - Triggered if the vehicle velocity equals or transcends the speed of light.
/// * `SpacetimeError::NumericalPrecisionLoss` - Triggered if the precision bounds collapse into a NaN or Infinite float layout.
pub fn compute_lorentz_time_dilation(velocity_vectors_ms: f64, delta_time_seconds: f64) -> SpacetimeResult<f64> {
    // 1. Guard check for physical boundaries: Velocity cannot equal or transcend the speed of light in a vacuum
    if velocity_vectors_ms >= SPEED_OF_LIGHT {
        return Err(SpacetimeError::SuperluminalVelocityViolation);
    }
    
    // 2. Prevent negative metrics from injecting unstable telemetry baselines
    let abs_velocity = velocity_vectors_ms.abs();

    // 3. Compute beta squared: (v / c)^2
    let beta_sq = (abs_velocity * abs_velocity) / (SPEED_OF_LIGHT * SPEED_OF_LIGHT);
    
    // 4. Compute Lorentz factor (gamma): 1 / sqrt(1 - beta^2)
    let lorentz_gamma = 1.0 / (1.0 - beta_sq).sqrt();
    
    // 5. Assert numerical stability for high-precision floating point contexts
    if lorentz_gamma.is_nan() || lorentz_gamma.is_infinite() {
        return Err(SpacetimeError::NumericalPrecisionLoss);
    }

    // 6. Return the absolute time drift displacement: delta_t' = delta_t * (gamma - 1)
    Ok(delta_time_seconds * (lorentz_gamma - 1.0))
}
