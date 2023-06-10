use std::collections::VecDeque;

use egui::plot::{Line, Plot, PlotPoints, Points};
use egui::{Align2, CentralPanel, SidePanel, Slider, TopBottomPanel};
use glam::{dvec3, DVec3, Vec3Swizzles};

use crate::lorenz::Lorenz;

const MAX_ITERATION: usize = 10000;

#[derive(PartialEq)]
enum Projection {
    XY,
    XZ,
    YZ,
}

struct LorenzState {
    lorenz: Lorenz,
    state: DVec3,
    history: VecDeque<DVec3>,
}

pub struct State {
    number_iterations: usize,
    lorenz_state: LorenzState,
    delta: f64,
    projection: Projection,
    last_time: f64,
    dynamic: bool,
}
impl State {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            number_iterations: MAX_ITERATION,
            lorenz_state: LorenzState {
                lorenz: Lorenz {
                    rho: 28.,
                    sigma: 10.,
                    beta: 8. / 3.,
                },
                state: dvec3(1., 0., 0.),
                history: VecDeque::new(),
            },
            delta: 0.001,
            projection: Projection::XZ,
            last_time: 0.,
            dynamic: true,
        }
    }
}
impl eframe::App for State {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let dt = {
            let t = ctx.input(|i| i.time);
            let dt = t - self.last_time;
            self.last_time = t;
            dt.clamp(0., 1. / 30.)
        };

        egui::Window::new("Configuration")
            .anchor(Align2::LEFT_TOP, (10., 10.))
            .resizable(false)
            .show(ctx, |ui| {
                TopBottomPanel::top("sliders").show_inside(ui, |ui| {
                    ui.style_mut().spacing.slider_width = 300.;

                    ui.add(
                        Slider::new(
                            &mut self.lorenz_state.lorenz.rho,
                            0. ..=100.,
                        )
                        .text("ρ"),
                    );
                    ui.add(
                        Slider::new(
                            &mut self.lorenz_state.lorenz.sigma,
                            0. ..=100.,
                        )
                        .text("σ"),
                    );
                    ui.add(
                        Slider::new(
                            &mut self.lorenz_state.lorenz.beta,
                            0. ..=100.,
                        )
                        .text("β"),
                    );
                    ui.add(
                        Slider::new(
                            &mut self.number_iterations,
                            1..=MAX_ITERATION,
                        )
                        .text("N"),
                    );
                    ui.add(
                        Slider::new(&mut self.delta, 0.001..=0.01).text("Δt"),
                    );
                });

                TopBottomPanel::top("projections").show_inside(ui, |ui| {
                    SidePanel::left("projection").show_inside(ui, |ui| {
                        ui.label("Projection");
                        ui.radio_value(
                            &mut self.projection,
                            Projection::XY,
                            "XY",
                        );
                        ui.radio_value(
                            &mut self.projection,
                            Projection::XZ,
                            "XZ",
                        );
                        ui.radio_value(
                            &mut self.projection,
                            Projection::YZ,
                            "YZ",
                        );
                    });
                    SidePanel::left("dynamic").show_inside(ui, |ui| {
                        ui.checkbox(&mut self.dynamic, "Dynamic");
                        if ui.button("Clear").clicked() {
                            self.lorenz_state.history.clear();
                        }
                        if ui.button("Reset").clicked() {
                            self.lorenz_state.history.clear();
                            self.lorenz_state.state = dvec3(1., 0., 0.);
                        }
                    });
                });
                if self.dynamic {
                    self.dynamic_plot(ctx, dt);
                } else {
                    self.static_plot(ctx);
                }
            });
    }
}

impl State {
    fn dynamic_plot(&mut self, ctx: &egui::Context, dt: f64) {
        for _ in 0..10 {
            self.lorenz_state
                .history
                .push_front(self.lorenz_state.state);
            self.lorenz_state.history.truncate(100000);
            self.lorenz_state.state += self
                .lorenz_state
                .lorenz
                .step(0.1 * dt, self.lorenz_state.state);
        }

        let proj = match self.projection {
            Projection::XY => DVec3::xy,
            Projection::XZ => DVec3::xz,
            Projection::YZ => DVec3::yz,
        };

        let line = Line::new(
            self.lorenz_state
                .history
                .iter()
                .map(|v| proj(*v).to_array())
                .collect::<PlotPoints>(),
        );

        let points = Points::new(PlotPoints::from(
            proj(self.lorenz_state.state).to_array(),
        ))
        .radius(4.);

        CentralPanel::default().show(ctx, |ui| {
            Plot::new("plot")
                .auto_bounds_x()
                .auto_bounds_y()
                .data_aspect(1.0)
                .show(ui, |pui| {
                    pui.line(line);
                    pui.points(points);
                });
        });

        ctx.request_repaint();
    }

    fn static_plot(&self, ctx: &egui::Context) {
        let iteration_factor = (1. / self.delta) as usize / 100;
        let mut state = dvec3(1., 0., 0.);
        let mut points = vec![];
        for _ in 0..self.number_iterations * iteration_factor {
            points.push(state);
            state += self.lorenz_state.lorenz.step(self.delta, state);
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
