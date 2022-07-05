// Constants: INFINITY, PI

pub use rand::prelude::*;
pub use std::f64::consts::PI;
pub use std::f64::INFINITY;

// Utility Functions

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

// Returns a random real in [0,1).
pub fn random_double() -> f64 {
    rand::random::<f64>()
}
// Returns a random real in [a,b).
pub fn random_double_in_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}
