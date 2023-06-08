use eframe::Theme;
use egui::Slider;

#[cfg(not(target_arch = "wasm32"))]
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
        "Lorenz",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    )
    .unwrap();
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas",
                web_options,
                Box::new(|cc| Box::new(MyEguiApp::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}

pub struct MyEguiApp {
    slider: f32,
}
impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self { slider: 0. }
    }
}
impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(Slider::new(&mut self.slider, -10.0..=100.0))
        });
    }
}