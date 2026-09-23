//! Astrophysical, cosmological, and physical constants in high-precision IEEE 754 float layouts.

/// Speed of light in a vacuum (meters per second).
pub const SPEED_OF_LIGHT: f64 = 299_792_458.0;

/// Universal Newtonian Gravitational Constant (m^3 * kg^-1 * s^-2).
pub const GRAVITATIONAL_CONSTANT: f64 = 6.674_30e-11;

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
