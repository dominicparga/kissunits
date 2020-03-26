pub mod units {
    use kissunits::{distance::Kilometers, speed::KilometersPerHour, time::Hours};

    // correct defaults for calculations
    pub const DURATION: Hours = Hours(2.0);
    pub const DISTANCE: Kilometers = Kilometers(72.0);
    pub const SPEED: KilometersPerHour = KilometersPerHour(36.0);

    pub fn approx_eq(a: f64, b: f64) -> bool {
        a == b
        // depends on the const-numbers above:
        // (a - b).abs() <= 2.0 * std::f64::EPSILON
    }
}
