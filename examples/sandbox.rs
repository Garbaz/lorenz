use glam::DVec3;
use lorenz::lorenz::Lorenz;

fn main() {
    let mut state = DVec3::from((1., 0., 0.));
    let dt = 0.01;

    let lorry = Lorenz {
        rho: 28.,
        sigma: 10.,
        beta: 8. / 3.,
    };

    for _ in 0..10 {
        state = state + lorry.step(dt, state);

        println!("{:?}", state);
    }
}
