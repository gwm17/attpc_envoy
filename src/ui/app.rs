use super::config::Config;
use crate::envoy::embassy::{Embassy, connect_embassy};
use crate::envoy::status_manager::StatusManager;
use crate::envoy::error::EmbassyError;
use eframe::egui::{RichText, Color32};

#[derive(Debug)]
pub struct EnvoyApp {
    config: Config,
    runtime: tokio::runtime::Runtime,
    embassy: Option<Embassy>,
    ecc_handles: Option<Vec<tokio::task::JoinHandle<()>>>,
    status: StatusManager
}

impl EnvoyApp {
    /// Startup the application
    pub fn new(_cc: &eframe::CreationContext<'_>, runtime: tokio::runtime::Runtime) -> Self {
        EnvoyApp { config: Config::new(), runtime, embassy: None, ecc_handles: None, status: StatusManager::new() }
    }

    fn connect(&mut self) {
        if self.embassy.is_none() && self.ecc_handles.is_none() {
            let (em, ecc_handles) = connect_embassy(&mut self.runtime, &self.config.experiment);
            tracing::info!("Connnected with {} tasks spawned", ecc_handles.len());
            self.embassy = Some(em);
            self.ecc_handles = Some(ecc_handles);
        }
    }

    fn disconnect(&mut self) {
        if self.embassy.is_some() {
            let mut embassy = self.embassy.take().expect("Literally cant happen");
            embassy.shutdown();
            let handles = self.ecc_handles.take().expect("Handles did not exist at disconnect?");
            for handle in handles {
                match self.runtime.block_on(handle) {
                    Ok(()) => (),
                    Err(e) => tracing::error!("Encountered an error whilst disconnecting: {}", e)
                }
            }
            tracing::info!("Disconnected the embassy");
            self.status.reset();
            tracing::info!("Status manager reset.")
        }
    }

    fn poll_embassy(&mut self) {
        if let Some(embassy) = self.embassy.as_mut() {
            match embassy.poll_messages() {
                Ok(messages) => {
                    match self.status.handle_messages(messages) {
                        Ok(_) => (),
                        Err(e) => tracing::error!("StatusManager ran into an error handling messages: {}", e)
                    };
                }
                Err(e) => tracing::error!("Embassy ran into an error polling the envoys: {}", e)
            };
        }
    }
}

impl eframe::App for EnvoyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        
        eframe::egui::CentralPanel::default().show(ctx, |ui| {

            //Probably don't want to poll every frame, but as a test...
            self.poll_embassy();

            ui.menu_button("File", |ui| {
                if ui.button("Save").clicked() {
                    println!("Saved");
                }
                if ui.button("Open").clicked() {
                    println!("Opened");
                }
            });

            ui.separator();
            ui.label(RichText::new("Configuration").color(Color32::LIGHT_BLUE).size(18.0));
            ui.label(RichText::new("Experiment").color(Color32::WHITE).size(12.0));
            ui.text_edit_singleline(&mut self.config.experiment);
            ui.label(RichText::new("Description").color(Color32::WHITE).size(12.0));
            ui.text_edit_singleline(&mut self.config.description);
            ui.label(RichText::new("Run Number").color(Color32::WHITE).size(12.0));
            ui.add(eframe::egui::widgets::DragValue::new(&mut self.config.run_number).speed(1));
            if ui.button(RichText::new("Connect").color(Color32::LIGHT_BLUE).size(14.0)).clicked() {
                self.connect();
            }
            if ui.button(RichText::new("Disconnect").color(Color32::LIGHT_RED).size(14.0)).clicked() {
                self.disconnect();
            }

        });

    }
}