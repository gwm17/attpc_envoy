mod ui;
mod envoy;

use ui::app::EnvoyApp;

fn main() {

    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(eframe::epaint::vec2(600.0, 300.0));
    native_options.follow_system_theme = false;
    match eframe::run_native("ATTPC Envoy", native_options, Box::new(|cc| Box::new(EnvoyApp::new(cc)))) {
        Ok(()) => (),
        Err(e) => println!("Eframe error: {}", e)
    }
    return;
}