use crate::vec::V3;

pub struct Lorenz {
    pub rho: f64,
    pub sigma: f64,
    pub beta: f64,
}

impl Lorenz {
    fn delta(&self, state: V3) -> V3 {
        let V3 { x, y, z } = state;
        V3 {
            x: self.sigma * (y - x),
            y: x * (self.rho - z) - y,
            z: x * y - self.beta * z,
        }
    }

    pub fn step(&self, dt: f64, state: V3) -> V3 {
        dt * self.delta(state)
    }
}
