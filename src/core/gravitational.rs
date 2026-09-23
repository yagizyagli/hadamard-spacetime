//! General Relativistic time expansion models based on Schwarzschild metrics and frame-dragging effects.

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
pub fn compute_schwarzschild_dilation(
    celestial_mass_kg: f64,
    radial_distance_meters: f64,
    delta_time_seconds: f64,
) -> SpacetimeResult<f64> {
    let c_sq = SPEED_OF_LIGHT * SPEED_OF_LIGHT;
    let schwarzschild_radius = (2.0 * GRAVITATIONAL_CONSTANT * celestial_mass_kg) / c_sq;

    if radial_distance_meters <= schwarzschild_radius {
        return Err(SpacetimeError::SchwarzschildRadiusBreach);
    }

    let metric_factor = (1.0 - (schwarzschild_radius / radial_distance_meters)).sqrt();
    
    if metric_factor.is_nan() {
        return Err(SpacetimeError::NumericalPrecisionLoss);
    }

    Ok(delta_time_seconds * (1.0 - metric_factor))
}

/// Computes the relativistic frame-dragging (Lense-Thirring effect) time drift 
/// induced by the angular momentum of a rotating celestial body.
///
/// # Arguments
///
/// * `angular_momentum_j` - Total angular momentum of the rotating planet/star (kg * m^2 / s).
/// * `radial_distance_meters` - Distance from the rotating anchor's core center (meters).
/// * `inclination_rad` - Orbital inclination relative to the rotational equator (radians).
/// * `delta_time_seconds` - Base execution telemetry window duration (seconds).
pub fn compute_lense_thirring_drift(
    angular_momentum_j: f64,
    radial_distance_meters: f64,
    inclination_rad: f64,
    delta_time_seconds: f64,
) -> SpacetimeResult<f64> {
    if radial_distance_meters <= 0.0 {
        return Err(SpacetimeError::SchwarzschildRadiusBreach);
    }

    let c_cube = SPEED_OF_LIGHT * SPEED_OF_LIGHT * SPEED_OF_LIGHT;
    let r_cube = radial_distance_meters * radial_distance_meters * radial_distance_meters;

    // Lense-Thirring precession rate omega = (2 * G * J) / (c^2 * r^3)
    let precession_rate = (2.0 * GRAVITATIONAL_CONSTANT * angular_momentum_j) / (SPEED_OF_LIGHT * SPEED_OF_LIGHT * r_cube);
    
    // Time displacement contribution adjusted by orbital inclination metrics
    let accumulated_drift = precession_rate * inclination_rad.cos() * delta_time_seconds;

    if accumulated_drift.is_nan() || accumulated_drift.is_infinite() {
        return Err(SpacetimeError::NumericalPrecisionLoss);
    }

    Ok(accumulated_drift)
}
