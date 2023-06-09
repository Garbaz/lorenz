use glam::DVec3;

pub struct Lorenz {
    pub rho: f64,
    pub sigma: f64,
    pub beta: f64,
}

impl Lorenz {
    fn delta(&self, state: DVec3) -> DVec3 {
        let DVec3 { x, y, z } = state;
        DVec3 {
            x: self.sigma * (y - x),
            y: x * (self.rho - z) - y,
            z: x * y - self.beta * z,
        }
    }

    pub fn step(&self, dt: f64, state: DVec3) -> DVec3 {
        dt * self.delta(state)
    }
}
