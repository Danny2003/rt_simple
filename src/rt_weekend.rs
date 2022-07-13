/// Utility Functions
pub use rand::prelude::*;
pub use std::f64::consts::PI;
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
/// Returns a random real in [0,1).
pub fn random_double() -> f64 {
    rand::random::<f64>()
}
/// Returns a random real in [a,b).
pub fn random_double_in_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}
pub fn random_int_in_range(min: i32, max: i32) -> i32 {
    random_double_in_range(min as f64, max as f64 + 1.) as i32
}
