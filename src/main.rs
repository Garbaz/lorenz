use eframe::Theme;
use lorenz::app::MyEguiApp;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let native_options = eframe::NativeOptions {
        default_theme: Theme::Dark,
        follow_system_theme: false,
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
