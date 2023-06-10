use egui::plot::{Line, Plot, PlotPoints};
use egui::{Align2, CentralPanel, Slider, TopBottomPanel};
use glam::{dvec3, DVec3, Vec3Swizzles};

use crate::lorenz::Lorenz;

const MAX_ITERATION: usize = 10000;

#[derive(PartialEq)]
enum Projection {
    XY,
    XZ,
    YZ,
}

pub struct State {
    number_iterations: usize,
    lorenz_attractor: Lorenz,
    delta: f64,
    projection: Projection,
}
impl State {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            number_iterations: MAX_ITERATION,
            lorenz_attractor: Lorenz {
                rho: 28.,
                sigma: 10.,
                beta: 8. / 3.,
            },
            delta: 0.001,
            projection: Projection::XZ,
        }
    }
}
impl eframe::App for State {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("Configuration")
            .anchor(Align2::LEFT_TOP, (10., 10.))
            .resizable(false)
            .show(ctx, |ui| {
                TopBottomPanel::top("sliders").show_inside(ui, |ui| {
                    ui.style_mut().spacing.slider_width = 300.;
                    ui.add(
                        Slider::new(
                            &mut self.number_iterations,
                            1..=MAX_ITERATION,
                        )
                        .text("N"),
                    );
                    ui.add(
                        Slider::new(&mut self.lorenz_attractor.rho, 0. ..=100.)
                            .text("ρ"),
                    );
                    ui.add(
                        Slider::new(
                            &mut self.lorenz_attractor.sigma,
                            0. ..=100.,
                        )
                        .text("σ"),
                    );
                    ui.add(
                        Slider::new(
                            &mut self.lorenz_attractor.beta,
                            0. ..=100.,
                        )
                        .text("β"),
                    );
                    ui.add(
                        Slider::new(&mut self.delta, 0.001..=0.01).text("Δt"),
                    );
                });

                TopBottomPanel::top("projections").show_inside(ui, |ui| {
                    ui.label("Projection");
                    ui.radio_value(&mut self.projection, Projection::XY, "XY");
                    ui.radio_value(&mut self.projection, Projection::XZ, "XZ");
                    ui.radio_value(&mut self.projection, Projection::YZ, "YZ");
                });
            });
        let iteration_factor = (1. / self.delta) as usize / 100;
        let mut state = dvec3(1., 0., 0.);
        let mut points = vec![];
        for _ in 0..self.number_iterations * iteration_factor {
            points.push(state);
            state += self.lorenz_attractor.step(self.delta, state);
        }

        let proj = match self.projection {
            Projection::XY => DVec3::xy,
            Projection::XZ => DVec3::xz,
            Projection::YZ => DVec3::yz,
        };
        let plot_points: PlotPoints =
            points.iter().map(|pos| proj(*pos).to_array()).collect();
        let line = Line::new(plot_points);

        CentralPanel::default().show(ctx, |ui| {
            Plot::new("plot")
                .auto_bounds_x()
                .auto_bounds_y()
                .data_aspect(1.0)
                .show(ui, |pui| pui.line(line));
        });
    }
}
