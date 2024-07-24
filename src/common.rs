use rand::random;

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;

// utility functions
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_f64() -> f64 {
    random::<f64>()
}

pub fn random_range_f64(min: f64, max: f64) -> f64{
    min + (max-min)*random_f64()
}