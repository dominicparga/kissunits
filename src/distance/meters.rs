use crate::{distance::Kilometers, speed::MetersPerSecond, time::Seconds};
use std::{
    fmt,
    fmt::Display,
    ops::{Add, AddAssign, Deref, DerefMut, Div, Sub, SubAssign},
};

#[derive(Debug, Default, Clone, Copy, PartialOrd, PartialEq)]
pub struct Meters(pub f64);

impl Meters {
    pub fn new(m: f64) -> Meters {
        Meters(m)
    }
}

impl Display for Meters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} m", self.0)
    }
}

impl From<Kilometers> for Meters {
    fn from(kilometers: Kilometers) -> Meters {
        Meters(kilometers.0 * 1_000.0)
    }
}

impl From<&Kilometers> for Meters {
    fn from(kilometers: &Kilometers) -> Meters {
        Meters::from(*kilometers)
    }
}

impl Deref for Meters {
    type Target = f64;

    fn deref(&self) -> &f64 {
        &self.0
    }
}

impl DerefMut for Meters {
    fn deref_mut(&mut self) -> &mut f64 {
        &mut self.0
    }
}

impl Add<Meters> for Meters {
    type Output = Meters;

    fn add(self, other: Meters) -> Meters {
        Meters(self.0 + other.0)
    }
}

impl AddAssign<Meters> for Meters {
    fn add_assign(&mut self, other: Meters) {
        self.0 += other.0;
    }
}

impl Sub<Meters> for Meters {
    type Output = Meters;

    fn sub(self, other: Meters) -> Meters {
        Meters(self.0 - other.0)
    }
}

impl SubAssign<Meters> for Meters {
    fn sub_assign(&mut self, other: Meters) {
        self.0 -= other.0;
    }
}

/// mps = m / s
impl Div<Seconds> for Meters {
    type Output = MetersPerSecond;

    fn div(self, duration: Seconds) -> MetersPerSecond {
        MetersPerSecond((*self) / (*duration))
    }
}

/// s = m / mps
impl Div<MetersPerSecond> for Meters {
    type Output = Seconds;

    fn div(self, speed: MetersPerSecond) -> Seconds {
        Seconds((*self) / (*speed))
    }
}
