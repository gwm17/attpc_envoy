use super::config::Config;
use crate::envoy::embassy::{Embassy, connect_embassy};
use crate::envoy::status_manager::StatusManager;
use crate::envoy::ecc_operation::ECCStatus;
use crate::envoy::surveyor_state::SurveyorState;
use crate::envoy::error::EmbassyError;
use eframe::egui::{RichText, Color32};
use reqwest::header;

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

    fn transition_ecc(&mut self, forward_transitions: Vec<usize>, backward_transitions: Vec<usize>) {
        todo!();
    }

    fn forward_transition_all(&mut self) {
        todo!();
    }

    fn backward_transition_all(&mut self) {
        todo!();
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

            ui.separator();
            ui.label(RichText::new("ECC Envoy Status/Control").color(Color32::LIGHT_BLUE).size(18.0));

            ui.horizontal(|ui| {
                ui.label(RichText::new(format!("System Status: {}", ECCStatus::from(0))).size(16.0).color(Color32::GOLD));
                ui.label(RichText::new("Regress system").size(16.0));
                if ui.button(RichText::new("\u{25C0}").color(Color32::RED).size(16.0)).clicked() {
                    self.backward_transition_all();
                }
                ui.label(RichText::new("Progress system").size(16.0));
                if ui.button(RichText::new("\u{25B6}").color(Color32::GREEN).size(16.0)).clicked() {
                    self.forward_transition_all();
                }
            });
            
            let mut forward_transitions: Vec<usize> = vec![];
            let mut backward_transitions: Vec<usize> = vec![];

            ui.push_id(0, |ui| {
                egui_extras::TableBuilder::new(ui)
                    .striped(true)
                    .column(egui_extras::Column::auto().at_least(50.0).resizable(true))
                    .column(egui_extras::Column::auto().at_least(50.0).resizable(true))
                    .column(egui_extras::Column::auto().at_least(50.0).resizable(true))
                    .column(egui_extras::Column::auto().at_least(50.0).resizable(true))
                    .header(40.0, |mut header| {
                        header.col(|ui| {
                            ui.heading("Envoy");
                        });
                        header.col(|ui| {
                            ui.heading("Status");
                        });
                        header.col(|ui| {
                            ui.heading("Regress");
                        });
                        header.col(|ui| {
                            ui.heading("Progress");
                        });
                    })
                    .body(|body| {
                        let ecc_status = self.status.get_ecc_status();
                        body.rows(40.0, ecc_status.len(), |ridx, mut row| {
                            let status = &ecc_status[ridx];
                            row.col(|ui| {
                                ui.label(RichText::new(format!("ECC Envoy {}", ridx)).color(Color32::LIGHT_GREEN));
                            });
                            row.col(|ui| {
                                ui.label(RichText::new(format!("{}", ECCStatus::from(status.state))).color(Color32::GOLD));
                            });
                            row.col(|ui| {
                                if ui.button(RichText::new("\u{25C0}").color(Color32::RED)).clicked() {
                                    forward_transitions.push(ridx);
                                }
                            });
                            row.col(|ui| {
                                if ui.button(RichText::new("\u{25B6}").color(Color32::GREEN)).clicked() {
                                    backward_transitions.push(ridx);
                                }
                            });
                        });
                    });
            });
            

            ui.separator();
            ui.label(RichText::new("Data Router Status").color(Color32::LIGHT_BLUE).size(18.0));
            ui.label(RichText::new(format!("System Status: {}", SurveyorState::from(0))).color(Color32::GOLD).size(16.0));

            ui.separator();
            ui.push_id(1, |ui| {
                egui_extras::TableBuilder::new(ui)
                    .striped(true)
                    .column(egui_extras::Column::auto().at_least(50.0).resizable(true))
                    .column(egui_extras::Column::auto().at_least(50.0).resizable(true))
                    .column(egui_extras::Column::auto().at_least(80.0).resizable(true))
                    .column(egui_extras::Column::auto().at_least(100.0).resizable(true))
                    .column(egui_extras::Column::auto().at_least(50.0).resizable(true))
                    .column(egui_extras::Column::auto().at_least(100.0).resizable(true))
                    .column(egui_extras::Column::auto().at_least(100.0).resizable(true))
                    .column(egui_extras::Column::auto().at_least(100.0).resizable(true))
                    .column(egui_extras::Column::auto().at_least(100.0).resizable(true))
                    .header(40.0, |mut header| {
                        header.col(|ui| {
                            ui.heading("Envoy");
                        });
                        header.col(|ui| {
                            ui.heading("Status");
                        });
                        header.col(|ui| {
                            ui.heading("Location");
                        });
                        header.col(|ui| {
                            ui.heading("Disk Status");
                        });
                        header.col(|ui| {
                            ui.heading("Files");
                        });
                        header.col(|ui| {
                            ui.heading("Bytes Written");
                        });
                        header.col(|ui| {
                            ui.heading("Data Rate (MB/s)");
                        });
                        header.col(|ui| {
                            ui.heading("%Disk Used");
                        });
                        header.col(|ui| {
                            ui.heading("Disk Size");
                        });
                    })
                    .body(|body| {
                        let surveyor_status = self.status.get_surveyor_status();
                        body.rows(40.0, surveyor_status.len(), |ridx, mut row| {
                            let status = &surveyor_status[ridx];
                            row.col(|ui| {
                                ui.label(RichText::new(format!("Data Router {}", ridx)).color(Color32::LIGHT_GREEN));
                            });
                            row.col(|ui| {
                                ui.label(RichText::new(format!("{}", SurveyorState::from(status.state))).color(Color32::GOLD));
                            });
                            row.col(|ui| {
                                ui.label(RichText::new(status.location.clone()));
                            });
                            row.col(|ui| {
                                ui.label(RichText::new(status.disk_status.clone()));
                            });
                            row.col(|ui| {
                                ui.label(RichText::new(format!("{}", status.files)));
                            });
                            row.col(|ui| {
                                ui.label(RichText::new(format!("{}", human_bytes::human_bytes(status.bytes_used as f64))));
                            });
                            row.col(|ui| {
                                ui.label(RichText::new(format!("{}", status.data_rate)));
                            });
                            row.col(|ui| {
                                ui.label(RichText::new(status.percent_used.clone()));
                            });
                            row.col(|ui| {
                                ui.label(RichText::new(format!("{}", human_bytes::human_bytes(status.disk_space as f64))));
                            });
                        })
                    });
            });
                
            ui.separator();
        });

    }
}