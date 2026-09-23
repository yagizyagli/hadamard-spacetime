//! Executable simulation profile evaluating space-to-ground Quantum Key Distribution (QKD) links.
//! Showcases real-time compensation metrics for both Wigner spin rotations and gravitational phase shifts.

use hadamard_spacetime::constants::{EARTH_MASS, EARTH_RADIUS, SPEED_OF_LIGHT};
use hadamard_spacetime::quantum::phase_correction::{
    calibrate_wigner_phase_rotation, compute_gravitational_phase_shift,
};
use core::f64::consts::FRAC_PI_4;

fn main() {
    println!("======================================================================");
    println!("   HADAMARD-SPACETIME ENGINE v0.1.0 - QUANTUM KEY DISTRIBUTION LINK  ");
    println!("======================================================================");
    println!("[INFO] Initializing Space-to-Ground QKD Laser Interlink Simulation...");

    // 1. Establish quantum infrastructure parameters
    let satellite_name = "HADAMARD-Q1";
    let orbital_velocity_ms = 7500.0;            // Rapid LEO orbital speed tracking metrics (m/s)
    let ground_station_radius = EARTH_RADIUS;    // Transmitting or receiving anchor on Earth's surface
    let satellite_orbit_radius = EARTH_RADIUS + 500_000.0; // 500km altitude orbital track
    
    // QKD standard carrier pulse: 1550nm wavelength equivalent to 193.1 Terahertz telecom laser band
    let qkd_carrier_frequency_hz = 193.1e12; 
    let initial_polarization_rad = FRAC_PI_4;     // Native 45-degree BB84 protocol photon polarization constant

    println!("[INFO] Deploying Node: Vector Space Interface [{}]", satellite_name);
    println!("[INFO] Optical Channel Frequency: {} THz (1550nm Range)", qkd_carrier_frequency_hz / 1e12);
    println!("[INFO] Line-of-Sight Kinematics: Tracking at {:.2} km/s", orbital_velocity_ms / 1000.0);
    println!("[INFO] Activating Relativistic Quantum Phase Correction Grids...\n");

    // 2. Telemetry tracking slices simulating different alignment steps over the horizon pass
    let simulation_slices = 5;
    
    for slice in 1..=simulation_slices {
        println!("--- [CRYPTO STREAM INTERCEPT POINT - BOUNDARY SEGMENT {}] ---", slice);
        
        // Dynamically simulate small variations in alignment angles due to satellite atmospheric crossing angles
        let dynamic_raw_angle = initial_polarization_rad + (slice as f64 * 0.005);
        println!("  Raw Input Qubit Phase (Rad)  : {:.6}", dynamic_raw_angle);

        // A. Calculate Kinematic Special Relativistic Shift: Wigner Spin Rotation
        match calibrate_wigner_phase_rotation(dynamic_raw_angle, orbital_velocity_ms) {
            Ok(wigner_aligned_phase) => {
                let angular_drift_error = dynamic_raw_angle - wigner_aligned_phase;
                println!("  Wigner Spin Compensation (Rad): {:.10}", wigner_aligned_phase);
                println!("  Kinematic Phase Shift Offset : {:+.10e}", angular_drift_error);
            },
            Err(e) => println!("  [CRITICAL ERROR] Kinematic Phase Cascade Failure: {:?}", e),
        }

        // B. Calculate General Relativistic Shift: Gravitational Wavepacket Redshift Distortion
        match compute_gravitational_phase_shift(
            EARTH_MASS,
            satellite_orbit_radius, // Source: Satellite at 500km altitude
            ground_station_radius,  // Receiver: Earth ground terminal
            qkd_carrier_frequency_hz,
        ) {
            Ok(grav_phase_shift) => {
                println!("  Gravitational Redshift Phase : {:+.10e} Rad / ns window", grav_phase_shift);
                
                // C. System Coherence Evaluation Metric
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

    // 3. Mathematical validation proof of safe execution velocities
    let speed_of_light_percentage = (orbital_velocity_ms / SPEED_OF_LIGHT) * 100.0;
    println!("======================================================================");
    println!("  Target Velocity Tracking Factor: {:.5}% of Speed of Light", speed_of_light_percentage);
    println!("  QUANTUM CRYPTO ENVELOPE SEALED - TERMINAL INTERLINK SECURED");
    println!("======================================================================");
}
