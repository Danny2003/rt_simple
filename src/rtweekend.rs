// Constants: INFINITY, PI

use std::f64::consts::PI;
use std::f64::INFINITY;

// Utility Functions

pub fn degrees_to_radians(degrees: f64) {
    degrees * PI / 180.0;
}
