use crate::{distance::Meters, speed::KilometersPerHour, time::Seconds};
use std::{
    fmt,
    fmt::Display,
    ops::{Add, AddAssign, Deref, DerefMut, Div, Mul, Sub, SubAssign},
};

#[derive(Debug, Default, Clone, Copy, PartialOrd, PartialEq)]
pub struct MetersPerSecond(pub f64);

impl Display for MetersPerSecond {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} m/s", self.0)
    }
}

impl From<KilometersPerHour> for MetersPerSecond {
    fn from(kmph: KilometersPerHour) -> MetersPerSecond {
        MetersPerSecond(kmph.0 / 3.6)
    }
}

impl From<&KilometersPerHour> for MetersPerSecond {
    fn from(kmph: &KilometersPerHour) -> MetersPerSecond {
        MetersPerSecond::from(*kmph)
    }
}

impl Deref for MetersPerSecond {
    type Target = f64;

    fn deref(&self) -> &f64 {
        &self.0
    }
}

impl DerefMut for MetersPerSecond {
    fn deref_mut(&mut self) -> &mut f64 {
        &mut self.0
    }
}

impl Add<MetersPerSecond> for MetersPerSecond {
    type Output = MetersPerSecond;

    fn add(self, other: MetersPerSecond) -> MetersPerSecond {
        MetersPerSecond(self.0 + other.0)
    }
}

impl AddAssign<MetersPerSecond> for MetersPerSecond {
    fn add_assign(&mut self, other: MetersPerSecond) {
        self.0 += other.0;
    }
}

impl Sub<MetersPerSecond> for MetersPerSecond {
    type Output = MetersPerSecond;

    fn sub(self, other: MetersPerSecond) -> MetersPerSecond {
        MetersPerSecond(self.0 - other.0)
    }
}

impl SubAssign<MetersPerSecond> for MetersPerSecond {
    fn sub_assign(&mut self, other: MetersPerSecond) {
        self.0 -= other.0;
    }
}

/// m = mps * s
impl Mul<Seconds> for MetersPerSecond {
    type Output = Meters;

    fn mul(self, duration: Seconds) -> Meters {
        Meters(self.0 * duration.0)
    }
}

/// f64 = mps / mps
impl Div<MetersPerSecond> for MetersPerSecond {
    type Output = f64;

    fn div(self, other: MetersPerSecond) -> f64 {
        self.0 / other.0
    }
}
