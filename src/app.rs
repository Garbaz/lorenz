use egui::plot::{Line, Plot, PlotPoints};
use egui::Slider;
use glam::{dvec3, DVec3, Vec3Swizzles};

use crate::lorenz::Lorenz;

pub struct MyEguiApp {
    slider: u64,
    lorenz_attractor: Lorenz,
}
impl MyEguiApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            slider: 0,
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
            ui.add(Slider::new(&mut self.slider, 0..=1000));
            let mut state = dvec3(1., 0., 0.);
            let mut points = vec![];
            for _ in 0..self.slider {
                points.push(state);
                state += self.lorenz_attractor.step(dt, state);
            }
            let plot_points: PlotPoints = points.iter().map(|pos| pos.xy().to_array()).collect();
            let line = Line::new(plot_points);
            Plot::new("my_plot").show(ui, |pui| pui.line(line));
        });
    }
}
