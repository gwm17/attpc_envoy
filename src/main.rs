mod command;
mod envoy;
mod ui;

use tokio::runtime::Builder;
use ui::app::EnvoyApp;

fn main() {
    //Create the async runtime
    let runtime: tokio::runtime::Runtime = Builder::new_multi_thread()
        .worker_threads(5)
        .enable_time()
        .enable_io()
        .build()
        .expect("Could not startup async runtime!");

    //Create our logging/tracing system.
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Could not initialize the tracing system!");

    tracing::info!("Tracing initialized!");

    //Start our application
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport = eframe::egui::ViewportBuilder::default()
        .with_title("AT-TPC Envoy")
        .with_inner_size(eframe::epaint::vec2(1400.0, 1225.0));
    native_options.follow_system_theme = false;
    match eframe::run_native(
        "ATTPC Envoy",
        native_options,
        Box::new(|cc| Box::new(EnvoyApp::new(cc, runtime))),
    ) {
        Ok(()) => (),
        Err(e) => tracing::error!("Eframe error: {}", e),
    }

    return;
}
