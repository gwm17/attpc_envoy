use super::config::Config;
use eframe::egui::{RichText, Color32};

#[derive(Debug)]
pub struct EnvoyApp {
    config: Config
}

impl EnvoyApp {
    /// Startup the application
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        EnvoyApp { config: Config::new() }
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