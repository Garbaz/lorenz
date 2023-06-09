use crate::vec::V;

pub struct Lorenz {
    rho: f64,
    sigma: f64,
    beta: f64,
}

impl Lorenz {
    pub fn delta(&self, state: V) -> V {
        let V { x, y, z } = state;
        V {
            x: self.sigma * (y - x),
            y: x * (self.rho - z) - y,
            z: x * y - self.beta * z,
        }
    }

    pub fn step(&self, dt: f64, state: V) -> V {
        dt * self.delta(state)
    }
}
