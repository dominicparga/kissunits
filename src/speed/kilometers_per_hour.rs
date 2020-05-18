use crate::{distance::Kilometers, speed::MetersPerSecond, time::Hours};
use std::{
    fmt,
    fmt::Display,
    ops::{Add, AddAssign, Deref, DerefMut, Div, Mul, Sub, SubAssign},
};

#[derive(Debug, Default, Clone, Copy, PartialOrd, PartialEq)]
pub struct KilometersPerHour(pub f64);

impl KilometersPerHour {
    pub fn new(kmph: f64) -> KilometersPerHour {
        KilometersPerHour(kmph)
    }
}

impl Display for KilometersPerHour {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} km/h", self.0)
    }
}

impl From<MetersPerSecond> for KilometersPerHour {
    fn from(mps: MetersPerSecond) -> KilometersPerHour {
        KilometersPerHour(mps.0 * 3.6)
    }
}

impl From<&MetersPerSecond> for KilometersPerHour {
    fn from(mps: &MetersPerSecond) -> KilometersPerHour {
        KilometersPerHour::from(*mps)
    }
}

impl Deref for KilometersPerHour {
    type Target = f64;

    fn deref(&self) -> &f64 {
        &self.0
    }
}

impl DerefMut for KilometersPerHour {
    fn deref_mut(&mut self) -> &mut f64 {
        &mut self.0
    }
}

impl Add<KilometersPerHour> for KilometersPerHour {
    type Output = KilometersPerHour;

    fn add(self, other: KilometersPerHour) -> KilometersPerHour {
        KilometersPerHour(self.0 + other.0)
    }
}

impl AddAssign<KilometersPerHour> for KilometersPerHour {
    fn add_assign(&mut self, other: KilometersPerHour) {
        self.0 += other.0;
    }
}

impl Sub<KilometersPerHour> for KilometersPerHour {
    type Output = KilometersPerHour;

    fn sub(self, other: KilometersPerHour) -> KilometersPerHour {
        KilometersPerHour(self.0 - other.0)
    }
}

impl SubAssign<KilometersPerHour> for KilometersPerHour {
    fn sub_assign(&mut self, other: KilometersPerHour) {
        self.0 -= other.0;
    }
}

/// km = kmph * h
impl Mul<Hours> for KilometersPerHour {
    type Output = Kilometers;

    fn mul(self, duration: Hours) -> Kilometers {
        Kilometers(self.0 * duration.0)
    }
}

/// f64 = kmph / kmph
impl Div<KilometersPerHour> for KilometersPerHour {
    type Output = f64;

    fn div(self, other: KilometersPerHour) -> f64 {
        self.0 / other.0
    }
}
