use std::ops::{Add, Mul};

pub struct V {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl From<(f64, f64, f64)> for V {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Self { x, y, z }
    }
}

impl Add for V {
    type Output = V;

    fn add(self, rhs: Self) -> Self::Output {
        V {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Mul<V> for f64 {
    type Output = V;

    fn mul(self, rhs: V) -> Self::Output {
        V {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul<f64> for V {
    type Output = V;

    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}
