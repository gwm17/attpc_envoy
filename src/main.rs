mod ui;
mod envoy;

use ui::app::EnvoyApp;
use envoy::message::{ECCMessage, EmbassyMessage};
use envoy::ecc_envoy::ECCEnvoy;
use envoy::embassy::Embassy;
use tokio::runtime::Builder;
use tokio::sync::mpsc::channel;

fn main() {

    let runtime: tokio::runtime::Runtime = Builder::new_multi_thread().worker_threads(1).build().expect("Could not startup async runtime!");

    let (ecc_tx, ecc_rx) = channel::<ECCMessage>(10);
    let (embassy_tx, embassy_rx) = channel::<EmbassyMessage>(10);


    let envoy_handle = runtime.spawn(async move {
        if let Ok(mut ev) = ECCEnvoy::new(String::from("envoy1"), embassy_rx, ecc_tx).await {
            match ev.wait_for_messages().await {
                Ok(()) => (),
                Err(e) => tracing::error!("Envoy {} ran into an error: {}", ev.get_id(), e)
            };
        } else {
            tracing::error!("Error creating envoy!");
        }
    });

    let embassy = Embassy::new(ecc_rx, vec![embassy_tx]);


    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(eframe::epaint::vec2(600.0, 300.0));
    native_options.follow_system_theme = false;
    match eframe::run_native("ATTPC Envoy", native_options, Box::new(|cc| Box::new(EnvoyApp::new(cc, embassy)))) {
        Ok(()) => (),
        Err(e) => println!("Eframe error: {}", e)
    }

    //Shutdown 
    runtime.block_on(envoy_handle).unwrap();

    return;
}