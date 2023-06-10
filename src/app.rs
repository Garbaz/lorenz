use egui::plot::{Line, Plot, PlotBounds, PlotPoints};
use egui::Slider;
use glam::{dvec3, DVec2, Vec3Swizzles};

use crate::lorenz::Lorenz;
const MAX_ITERATION: usize = 1000;
pub struct MyEguiApp {
    number_iterations: usize,
    lorenz_attractor: Lorenz,
}
impl MyEguiApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            number_iterations: 0,
            lorenz_attractor: Lorenz {
                rho: 28.,
                sigma: 10.,
                beta: 8. / 3.,
            },
        }
    }
}
impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // let dt = ctx.input(|is| is.stable_dt) as f64;
        let dt = 0.01;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().spacing.slider_width = 500.;
            ui.add(Slider::new(&mut self.number_iterations, 0..=MAX_ITERATION));
            ui.add(Slider::new(&mut self.lorenz_attractor.rho, 0. ..=100.));
            ui.add(Slider::new(&mut self.lorenz_attractor.sigma, 0. ..=100.));
            ui.add(Slider::new(&mut self.lorenz_attractor.beta, 0. ..=100.));

            let mut state = dvec3(1., 0., 0.);
            let mut points = vec![];
            for _ in 0..MAX_ITERATION {
                points.push(state);
                state += self.lorenz_attractor.step(dt, state);
            }
            let (min, max) = points.iter().fold(
                ([f64::MAX, f64::MAX], [0., 0.]),
                |acc, v| {
                    (
                        v.xy().min(acc.0.into()).to_array(),
                        v.xy().max(acc.1.into()).to_array(),
                    )
                },
            );
            let plot_points: PlotPoints = points[0..self.number_iterations]
                .iter()
                .map(|pos| pos.xy().to_array())
                .collect();
            let line = Line::new(plot_points);
            Plot::new("my_plot").show(ui, |pui| {
                pui.set_plot_bounds(PlotBounds::from_min_max(min, max));
                pui.line(line)
            });
        });
    }
}
