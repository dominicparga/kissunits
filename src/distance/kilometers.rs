use crate::{distance::Meters, speed::KilometersPerHour, time::Hours};
use std::{
    fmt,
    fmt::Display,
    ops::{Add, AddAssign, Deref, DerefMut, Div, Sub, SubAssign},
};

#[derive(Debug, Default, Clone, Copy, PartialOrd, PartialEq)]
pub struct Kilometers(pub f64);

impl Display for Kilometers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} km", self.0)
    }
}

impl From<Meters> for Kilometers {
    fn from(meters: Meters) -> Kilometers {
        Kilometers(meters.0 / 1_000.0)
    }
}

impl From<&Meters> for Kilometers {
    fn from(meters: &Meters) -> Kilometers {
        Kilometers::from(*meters)
    }
}

impl Deref for Kilometers {
    type Target = f64;

    fn deref(&self) -> &f64 {
        &self.0
    }
}

impl DerefMut for Kilometers {
    fn deref_mut(&mut self) -> &mut f64 {
        &mut self.0
    }
}

impl Add<Kilometers> for Kilometers {
    type Output = Kilometers;

    fn add(self, other: Kilometers) -> Kilometers {
        Kilometers(self.0 + other.0)
    }
}

impl AddAssign<Kilometers> for Kilometers {
    fn add_assign(&mut self, other: Kilometers) {
        self.0 += other.0;
    }
}

impl Sub<Kilometers> for Kilometers {
    type Output = Kilometers;

    fn sub(self, other: Kilometers) -> Kilometers {
        Kilometers(self.0 - other.0)
    }
}

impl SubAssign<Kilometers> for Kilometers {
    fn sub_assign(&mut self, other: Kilometers) {
        self.0 -= other.0;
    }
}

/// kmph = km / h
impl Div<Hours> for Kilometers {
    type Output = KilometersPerHour;

    fn div(self, duration: Hours) -> KilometersPerHour {
        KilometersPerHour(self.0 / duration.0)
    }
}

/// h = km / kmph
impl Div<KilometersPerHour> for Kilometers {
    type Output = Hours;

    fn div(self, speed: KilometersPerHour) -> Hours {
        Hours(self.0 / speed.0)
    }
}

/// f64 = km / km
impl Div<Kilometers> for Kilometers {
    type Output = f64;

    fn div(self, other: Kilometers) -> f64 {
        self.0 / other.0
    }
}
