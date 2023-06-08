fn main() {
    let native_options = eframe::NativeOptions {
        default_theme: Theme::Light,
        fullscreen: true,
        // initial_window_size: Some(egui::Vec2 {
        //     x: SCREEN_SIZE.0 as f32,
        //     y: SCREEN_SIZE.1 as f32,
        // }),
        multisampling: 4,
        ..Default::default()
    };
    eframe::run_native(
        "Visualization",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    )
    .unwrap();
}
pub struct MyEguiApp {}
impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {}
}
impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {});
    }
}
