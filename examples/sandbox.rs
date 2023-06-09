use std::time::Instant;

use lorenz::{lorenz::Lorenz, vec::V3};

fn main() {
    let mut state = V3::from((0.1, 0.1, 0.1));
    const DT: f64 = 0.1;

    const LORENZ: Lorenz = Lorenz {
        rho: 28.,
        sigma: 10.,
        beta: 8. / 3.,
    };

    for _ in 0..100 {
        let t = Instant::now();

        state = LORENZ.step(DT, state);

        println!("{:?}", state);
    }
}
