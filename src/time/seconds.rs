use crate::{
    distance::Meters,
    speed::MetersPerSecond,
    time::{Hours, Minutes},
};
use std::{
    fmt,
    fmt::Display,
    ops::{Add, AddAssign, Deref, DerefMut, Mul, Sub, SubAssign},
};

#[derive(Debug, Default, Clone, Copy, PartialOrd, PartialEq)]
pub struct Seconds(pub f64);

impl Seconds {
    pub fn new(s: f64) -> Seconds {
        Seconds(s)
    }
}

impl Display for Seconds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} s", self.0)
    }
}

impl From<Minutes> for Seconds {
    fn from(minutes: Minutes) -> Seconds {
        Seconds(minutes.0 * 60.0)
    }
}

impl From<&Minutes> for Seconds {
    fn from(minutes: &Minutes) -> Seconds {
        Seconds::from(*minutes)
    }
}

impl From<Hours> for Seconds {
    fn from(hours: Hours) -> Seconds {
        Seconds(hours.0 * 3_600.0)
    }
}

impl From<&Hours> for Seconds {
    fn from(hours: &Hours) -> Seconds {
        Seconds::from(*hours)
    }
}

impl Deref for Seconds {
    type Target = f64;

    fn deref(&self) -> &f64 {
        &self.0
    }
}

impl DerefMut for Seconds {
    fn deref_mut(&mut self) -> &mut f64 {
        &mut self.0
    }
}

impl Add<Seconds> for Seconds {
    type Output = Seconds;

    fn add(self, other: Seconds) -> Seconds {
        Seconds(self.0 + other.0)
    }
}

impl AddAssign<Seconds> for Seconds {
    fn add_assign(&mut self, other: Seconds) {
        self.0 += other.0;
    }
}

impl Sub<Seconds> for Seconds {
    type Output = Seconds;

    fn sub(self, other: Seconds) -> Seconds {
        Seconds(self.0 - other.0)
    }
}

impl SubAssign<Seconds> for Seconds {
    fn sub_assign(&mut self, other: Seconds) {
        self.0 -= other.0;
    }
}

/// m = s * mps
impl Mul<MetersPerSecond> for Seconds {
    type Output = Meters;

    fn mul(self, speed: MetersPerSecond) -> Meters {
        Meters(speed.0 * self.0)
    }
}
