//! Spaceborne QKD phase compensation mechanisms accounting for relativistic Wigner spin rotations and gravitational phase shifts.

use crate::constants::{GRAVITATIONAL_CONSTANT, SPEED_OF_LIGHT};
use crate::error::{SpacetimeError, SpacetimeResult};

/// Re-aligns qubit orbital polarization angles shifted via planetary velocity paths (Wigner Rotation).
/// Returns the calibrated quantum phase offset angle expressed in radians.
///
/// # Arguments
///
/// * `polarization_angle_rad` - Measured incoming raw quantum photon polarization vector angle (radians).
/// * `orbital_velocity_ms` - Instantaneous linear orbital traversal rate profile (meters / second).
pub fn calibrate_wigner_phase_rotation(
    polarization_angle_rad: f64,
    orbital_velocity_ms: f64,
) -> SpacetimeResult<f64> {
    if orbital_velocity_ms >= SPEED_OF_LIGHT {
        return Err(SpacetimeError::SuperluminalVelocityViolation);
    }
    
    let abs_velocity = orbital_velocity_ms.abs();
    let relativistic_beta = abs_velocity / SPEED_OF_LIGHT;
    let lorentz_contraction_factor = (1.0 - relativistic_beta.powi(2)).sqrt();
    
    if lorentz_contraction_factor < 1.0e-7 {
        return Err(SpacetimeError::QuantumDecoherenceCrisis);
    }

    let correction_factor = polarization_angle_rad * lorentz_contraction_factor;
    
    if correction_factor.is_nan() {
        return Err(SpacetimeError::NumericalPrecisionLoss);
    }

    Ok(correction_factor)
}

/// Computes the gravitational phase shift (gravitational redshift induction) 
/// affecting photon wavepackets traveling through a celestial gravitational well.
/// Returns the accumulated phase shift delta value in radians.
///
/// # Arguments
///
/// * `celestial_mass_kg` - Total mass of the local planetary/stellar gravity anchor (kilograms).
/// * `source_radius_meters` - Orbital radius where the quantum photon was emitted (meters).
/// * `receiver_radius_meters` - Orbital/ground radius where the photon is captured (meters).
/// * `photon_frequency_hz` - Baseline frequency of the underlying QKD carrier laser (Hertz).
pub fn compute_gravitational_phase_shift(
    celestial_mass_kg: f64,
    source_radius_meters: f64,
    receiver_radius_meters: f64,
    photon_frequency_hz: f64,
) -> SpacetimeResult<f64> {
    if source_radius_meters <= 0.0 || receiver_radius_meters <= 0.0 || photon_frequency_hz <= 0.0 {
        return Err(SpacetimeError::NumericalPrecisionLoss);
    }

    let c_sq = SPEED_OF_LIGHT * SPEED_OF_LIGHT;
    let factor_constant = (2.0 * GRAVITATIONAL_CONSTANT * celestial_mass_kg) / c_sq;

    // Evaluate Schwarzschild potential limits at emission and reception boundaries
    if source_radius_meters <= factor_constant || receiver_radius_meters <= factor_constant {
        return Err(SpacetimeError::SchwarzschildRadiusBreach);
    }

    let potential_source = (1.0 - (factor_constant / source_radius_meters)).sqrt();
    let potential_receiver = (1.0 - (factor_constant / receiver_radius_meters)).sqrt();

    // Gravitational frequency shift ratio (z + 1) = potential_receiver / potential_source
    let frequency_ratio = potential_receiver / potential_source;
    
    // Phase shift integration output over atomic clock pulse baseline: delta_phi = 2 * pi * f * (ratio - 1)
    use core::f64::consts::PI;
    let phase_shift_rad = 2.0 * PI * photon_frequency_hz * (frequency_ratio - 1.0) * 1.0e-9; // Scaled to nanosecond pulse windows

    if phase_shift_rad.is_nan() || phase_shift_rad.is_infinite() {
        return Err(SpacetimeError::NumericalPrecisionLoss);
    }

    Ok(phase_shift_rad)
}
