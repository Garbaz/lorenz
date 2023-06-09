use egui::Slider;
use egui::plot::Plot;

pub struct MyEguiApp {
    slider: f32,
}
impl MyEguiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self { slider: 0. }
    }
}
impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(Slider::new(&mut self.slider, -10.0..=100.0));

            ui.add(Plot::new(""));
        });

    }
}
