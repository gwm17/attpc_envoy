use super::config::Config;
use crate::envoy::{embassy::Embassy, error::EmbassyError, message};
use eframe::egui::{RichText, Color32};

#[derive(Debug)]
pub struct EnvoyApp {
    config: Config,
    embassy: Embassy
}

impl EnvoyApp {
    /// Startup the application
    pub fn new(_cc: &eframe::CreationContext<'_>, embassy: Embassy) -> Self {
        EnvoyApp { config: Config::new(), embassy: embassy }
    }

    fn poll_embassy(&mut self) -> Result<(), EmbassyError> {
        let messages = self.embassy.poll_messages()?;
        for message in messages {
            //do some stuff
            tracing::info!("We've got some messages: {}", message);
        }

        Ok(())
    }
}

impl eframe::App for EnvoyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        
        eframe::egui::CentralPanel::default().show(ctx, |ui| {

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

        });

    }
}