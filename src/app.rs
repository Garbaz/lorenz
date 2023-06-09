use egui::plot::{Line, Plot, PlotPoints};
use egui::{Pos2, Slider};

pub struct MyEguiApp {
    slider: f32,
    points: Vec<Pos2>,
}
impl MyEguiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            slider: 0.,
            points: vec![],
        }
    }
}
impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(Slider::new(&mut self.slider, -10.0..=100.0));
            let plot_points: PlotPoints = self
                .points
                .iter()
                .map(|pos| [pos.x as f64, pos.y as f64])
                .collect();
            let line = Line::new(plot_points);
            Plot::new("my_plot").show(ui, |pui| pui.line(line));
        });
    }
}
