use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy)]
pub struct V3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl<T: Into<f64>> From<(T, T, T)> for V3 {
    fn from((x, y, z): (T, T, T)) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }
}

impl<T: From<f64>> From<V3> for (T, T, T) {
    fn from(V3 { x, y, z }: V3) -> Self {
        (x.into(), y.into(), z.into())
    }
}

impl Add for V3 {
    type Output = V3;

    fn add(self, rhs: Self) -> Self::Output {
        V3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Mul<V3> for f64 {
    type Output = V3;

    fn mul(self, rhs: V3) -> Self::Output {
        V3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul<f64> for V3 {
    type Output = V3;

    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}
