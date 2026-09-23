//! Astrophysical, cosmological, and physical constants in high-precision IEEE 754 float layouts.
//! Covers the entire Solar System from the Sun out to Pluto for deep-space navigation networks.

/// Speed of light in a vacuum (meters per second).
pub const SPEED_OF_LIGHT: f64 = 299_792_458.0;

/// Universal Newtonian Gravitational Constant (m^3 * kg^-1 * s^-2).
pub const GRAVITATIONAL_CONSTANT: f64 = 6.674_30e-11;

/// Mass of the Sun (kilograms) - Crucial for heliocentric deep-space transfer phases.
pub const SUN_MASS: f64 = 1.989e30;

/// Nominal solar equatorial radius (meters).
pub const SUN_RADIUS: f64 = 696_342_000.0;

/// Mass of Planet Mercury (kilograms) - High relativistic precession anchor.
pub const MERCURY_MASS: f64 = 3.3011e23;

/// Mean radius of Planet Mercury (meters).
pub const MERCURY_RADIUS: f64 = 2_439_700.0;

/// Mass of Planet Venus (kilograms) - Strong inner-system gravitational well.
pub const VENUS_MASS: f64 = 4.8675e24;

/// Mean radius of Planet Venus (meters).
pub const VENUS_RADIUS: f64 = 6_051_800.0;

/// Mass of Planet Earth (kilograms).
pub const EARTH_MASS: f64 = 5.972_2e24;

/// Mean equatorial radius of Planet Earth (meters).
pub const EARTH_RADIUS: f64 = 6_371_000.0;

/// Mass of the Moon (kilograms) modeled for LunaNet validation targets.
pub const MOON_MASS: f64 = 7.342e22;

/// Mean volumetric radius of the Moon (meters).
pub const MOON_RADIUS: f64 = 1_737_400.0;

/// Mass of Planet Mars (kilograms).
pub const MARS_MASS: f64 = 6.39e23;

/// Mean radius of Planet Mars (meters).
pub const MARS_RADIUS: f64 = 3_389_500.0;

/// Mass of Planet Jupiter (kilograms) - Major outer-system gravitational perturbation source.
pub const JUPITER_MASS: f64 = 1.898e27;

/// Mean equatorial radius of Planet Jupiter (meters).
pub const JUPITER_RADIUS: f64 = 71_492_000.0;

/// Mass of Planet Saturn (kilograms).
pub const SATURN_MASS: f64 = 5.683e26;

/// Mean equatorial radius of Planet Saturn (meters).
pub const SATURN_RADIUS: f64 = 58_232_000.0;

/// Mass of Planet Uranus (kilograms).
pub const URANUS_MASS: f64 = 8.681e25;

/// Mean equatorial radius of Planet Uranus (meters).
pub const URANUS_RADIUS: f64 = 25_362_000.0;

/// Mass of Planet Neptune (kilograms).
pub const NEPTUNE_MASS: f64 = 1.024e26;

/// Mean equatorial radius of Planet Neptune (meters).
pub const NEPTUNE_RADIUS: f64 = 24_622_000.0;

/// Mass of Dwarf Planet Pluto (kilograms) - Boundary marker for outer heliosphere trajectories.
pub const PLUTO_MASS: f64 = 1.303e22;

/// Mean radius of Dwarf Planet Pluto (meters).
pub const PLUTO_RADIUS: f64 = 1_188_300.0;
