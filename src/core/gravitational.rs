//! General Relativistic time expansion models based on Schwarzschild metrics.

use crate::constants::GRAVITATIONAL_CONSTANT;
use crate::constants::SPEED_OF_LIGHT;
use crate::error::{SpacetimeError, SpacetimeResult};

/// Approximates the time dilation generated due to gravitational mass proximity variations.
/// Returns the accumulated time drift skew in seconds based on General Relativity.
///
/// # Arguments
///
/// * `celestial_mass_kg` - Total mass of the major gravity vector anchor (kilograms).
/// * `radial_distance_meters` - Distance from the anchor's volumetric core center (meters).
/// * `delta_time_seconds` - Base epoch duration segment evaluated (seconds).
///
/// # Errors
///
/// * `SpacetimeError::SchwarzschildRadiusBreach` - Triggered if the vehicle drops within the event horizon or celestial core.
/// * `SpacetimeError::NumericalPrecisionLoss` - Triggered if the floating point calculus falls into NaN layouts.
pub fn compute_schwarzschild_dilation(
    celestial_mass_kg: f64,
    radial_distance_meters: f64,
    delta_time_seconds: f64,
) -> SpacetimeResult<f64> {
    let c_sq = SPEED_OF_LIGHT * SPEED_OF_LIGHT;
    
    // 1. Compute the Schwarzschild radius (r_s = 2GM / c^2)
    let schwarzschild_radius = (2.0 * GRAVITATIONAL_CONSTANT * celestial_mass_kg) / c_sq;

    // 2. Guard against gravitational collapse or core boundary penetration
    if radial_distance_meters <= schwarzschild_radius {
        return Err(SpacetimeError::SchwarzschildRadiusBreach);
    }

    // 3. Compute Schwarzschild metric time dilation factor: sqrt(1 - r_s / r)
    let metric_factor = (1.0 - (schwarzschild_radius / radial_distance_meters)).sqrt();
    
    // 4. Validate numerical integrity before telemetry output
    if metric_factor.is_nan() {
        return Err(SpacetimeError::NumericalPrecisionLoss);
    }

    // 5. Return absolute gravitational drift displacement: delta_t_drift = delta_t * (1 - metric_factor)
    Ok(delta_time_seconds * (1.0 - metric_factor))
}
