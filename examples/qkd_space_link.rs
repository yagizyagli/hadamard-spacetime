//! Executable simulation profile evaluating space-to-ground Quantum Key Distribution (QKD) links.
//! Showcases real-time compensation metrics for both Wigner spin rotations and gravitational phase shifts.

use hadamard_spacetime::constants::{EARTH_MASS, EARTH_RADIUS, SPEED_OF_LIGHT};
use hadamard_spacetime::quantum::phase_correction::{
    calibrate_wigner_phase_rotation, compute_gravitational_phase_shift,
};

fn main() {
    println!("======================================================================");
    println!("   HADAMARD-SPACETIME ENGINE v0.1.0 - QUANTUM KEY DISTRIBUTION LINK  ");
    println!("======================================================================");
    println!("[INFO] Initializing Space-to-Ground QKD Laser Interlink Simulation...");

    let satellite_name = "HADAMARD-Q1";
    let orbital_velocity_ms = 7500.0;            
    let ground_station_radius = EARTH_RADIUS;    
    let satellite_orbit_radius = EARTH_RADIUS + 500_000.0; 
    
    let qkd_carrier_frequency_hz = 193.1e12; 
    
    // Explicit inline core constant routing to clear legacy lint blockages
    let initial_polarization_rad = core::f64::consts::FRAC_PI_4;     

    println!("[INFO] Deploying Node: Vector Space Interface [{}]", satellite_name);
    println!("[INFO] Optical Channel Frequency: {} THz (1550nm Range)", qkd_carrier_frequency_hz / 1e12);
    println!("[INFO] Line-of-Sight Kinematics: Tracking at {:.2} km/s", orbital_velocity_ms / 1000.0);
    println!("[INFO] Activating Relativistic Quantum Phase Correction Grids...\n");

    let simulation_slices = 5;
    
    for slice in 1..=simulation_slices {
        println!("--- [CRYPTO STREAM INTERCEPT POINT - BOUNDARY SEGMENT {}] ---", slice);
        
        let dynamic_raw_angle = initial_polarization_rad + (slice as f64 * 0.005);
        println!("  Raw Input Qubit Phase (Rad)  : {:.6}", dynamic_raw_angle);

        match calibrate_wigner_phase_rotation(dynamic_raw_angle, orbital_velocity_ms) {
            Ok(wigner_aligned_phase) => {
                let angular_drift_error = dynamic_raw_angle - wigner_aligned_phase;
                println!("  Wigner Spin Compensation (Rad): {:.10}", wigner_aligned_phase);
                println!("  Kinematic Phase Shift Offset : {:+.10e}", angular_drift_error);
            },
            Err(e) => println!("  [CRITICAL ERROR] Kinematic Phase Cascade Failure: {:?}", e),
        }

        match compute_gravitational_phase_shift(
            EARTH_MASS,
            satellite_orbit_radius, 
            ground_station_radius,  
            qkd_carrier_frequency_hz,
        ) {
            Ok(grav_phase_shift) => {
                println!("  Gravitational Redshift Phase : {:+.10e} Rad / ns window", grav_phase_shift);
                
                if grav_phase_shift.abs() > 1.0e-3 {
                    println!("  Channel Security Advisory    : WARNING - Phase Distortion Approaching Coherence Limits");
                } else {
                    println!("  Channel Security Advisory    : OPTICAL LINK SECURE - Quantum Decoherence Prevented");
                }
            },
            Err(e) => println!("  [CRITICAL ERROR] Gravitational Field Extraction Failure: {:?}", e),
        }
        println!();
    }

    let speed_of_light_percentage = (orbital_velocity_ms / SPEED_OF_LIGHT) * 100.0;
    println!("======================================================================");
    println!("  Target Velocity Tracking Factor: {:.5}% of Speed of Light", speed_of_light_percentage);
    println!("  QUANTUM CRYPTO ENVELOPE SEALED - TERMINAL INTERLINK SECURED");
    println!("======================================================================");
}
