use std::ops::{Add, Deref, Div, Mul, Sub};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct Promille(pub usize);

impl Promille {
    pub fn from_div(dividend: usize, divisor: usize) -> Promille {
        Promille((1_000 * dividend) / divisor)
    }
}

impl From<usize> for Promille {
    fn from(value: usize) -> Self {
        Promille(1_000 * value)
    }
}

impl Into<usize> for Promille {
    fn into(self) -> usize {
        self.0 / 1_000
    }
}

impl From<i64> for Promille {
    fn from(value: i64) -> Self {
        Promille((1_000 * value) as usize)
    }
}

impl From<i32> for Promille {
    fn from(value: i32) -> Self {
        Promille((1_000 * value) as usize)
    }
}

impl From<f64> for Promille {
    fn from(value: f64) -> Self {
        Promille((1_000.0 * value) as usize)
    }
}

impl From<f32> for Promille {
    fn from(value: f32) -> Self {
        Promille((1_000.0 * value) as usize)
    }
}

impl Deref for Promille {
    type Target = usize;

    fn deref(&self) -> &usize {
        &self.0
    }
}

impl Add for Promille {
    type Output = Promille;

    fn add(self, rhs: Promille) -> Promille {
        Promille(self.0 + rhs.0)
    }
}

impl Sub for Promille {
    type Output = Promille;

    fn sub(self, rhs: Promille) -> Promille {
        Promille(self.0 - rhs.0)
    }
}

impl Mul<Promille> for usize {
    type Output = Promille;

    fn mul(self, rhs: Promille) -> Promille {
        Promille(self * rhs.0)
    }
}

impl Mul<usize> for Promille {
    type Output = Promille;

    fn mul(self, rhs: usize) -> Promille {
        Promille(self.0 * rhs)
    }
}

impl Div<usize> for Promille {
    type Output = Promille;

    fn div(self, divisor: usize) -> Promille {
        Promille(self.0 / divisor)
    }
}
