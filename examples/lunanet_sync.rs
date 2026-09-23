//! Standardized executable simulation mimicking a full NASA LunaNet satellite telemetry track.
//! This integration example showcases all relativistic core and quantum modules working in sync.

use hadamard_spacetime::constants::{MOON_MASS, MOON_RADIUS};
use hadamard_spacetime::core::gravitational::compute_schwarzschild_dilation;
use hadamard_spacetime::core::kinematic::compute_lorentz_time_dilation;
use hadamard_spacetime::quantum::clock::evaluate_quantum_clock_skew;
use hadamard_spacetime::quantum::phase_correction::calibrate_wigner_phase_rotation;
use hadamard_spacetime::telemetry::sync_packet::RelativisticSyncPacket;

fn main() {
    println!("======================================================================");
    println!("   HADAMARD-SPACETIME ENGINE v0.1.0 - LUNANET DEEP SPACE TELEMETRY   ");
    println!("======================================================================");
    println!("[INFO] Initializing Lunar Orbit Navigation Vector Analysis Sequences...");

    let satellite_id: u32 = 8842;           
    let mut mission_clock_nanos: u64 = 0;   
    let simulation_steps_seconds = 10;     
    
    let lunar_orbital_velocity_ms = 1633.0; 
    let spacecraft_altitude_meters = MOON_RADIUS + 100_000.0; 

    let mut total_kinematic_drift_secs = 0.0;
    let mut total_gravitational_drift_secs = 0.0;

    // Use core library constant mapping directly to satisfy clippy precision constraints
    let raw_photon_phase_rad = core::f64::consts::FRAC_PI_4; 

    for step in 1..=simulation_steps_seconds {
        mission_clock_nanos += 1_000_000_000;
        let epoch_step_delta = 1.0; 

        let kinematic_skew = compute_lorentz_time_dilation(lunar_orbital_velocity_ms, epoch_step_delta)
            .unwrap_or(0.0);
        total_kinematic_drift_secs += kinematic_skew;

        let gravitational_skew = compute_schwarzschild_dilation(MOON_MASS, spacecraft_altitude_meters, epoch_step_delta)
            .unwrap_or(0.0);
        total_gravitational_drift_secs += gravitational_skew;

        let mock_gradient_tensor = 2.4e-10; 
        let raw_atomic_drift_fs = 12.5;     
        let calibrated_clock_skew = evaluate_quantum_clock_skew(mock_gradient_tensor, raw_atomic_drift_fs)
            .unwrap_or(raw_atomic_drift_fs);

        let compensated_wigner_phase = calibrate_wigner_phase_rotation(raw_photon_phase_rad, lunar_orbital_velocity_ms)
            .unwrap_or(raw_photon_phase_rad);

        let sync_packet = RelativisticSyncPacket {
            satellite_id,
            mission_epoch_nanos: mission_clock_nanos,
            accumulated_kinematic_skew_secs: total_kinematic_drift_secs,
            accumulated_gravitational_skew_secs: total_gravitational_drift_secs,
        };

        if let Ok(serialized_payload) = sync_packet.compile_packet_buffer() {
            println!("--- [FLIGHT BOUNDARY RECORD EPOCH STEP {}] ---", step);
            println!("  Time Stamp (Nanos)  : {}", sync_packet.mission_epoch_nanos);
            println!("  Kinematic Skew (s)  : {:+.15e}", sync_packet.accumulated_kinematic_skew_secs);
            println!("  Gravitational Skew(s): {:+.15e}", sync_packet.accumulated_gravitational_skew_secs);
            println!("  Quantum Clock Skew(fs): {:.4}", calibrated_clock_skew);
            println!("  Wigner Align (Rad)  : {:.6}", compensated_wigner_phase);
            println!("  Serialized Hex Frame: {:02X?}", &serialized_payload[..]);
        }
    }

    println!("\n======================================================================");
    println!("   TELEMETRY STREAM SUCCESSFULLY COMPLETE - VEHICLE NETWORK LOCKED   ");
    println!("======================================================================");
}
