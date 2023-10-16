use super::ecc_envoy::{ECCOperationResponse, ECCStatusResponse};
use super::ecc_operation::ECCStatus;
use super::surveyor_envoy::SurveyorResponse;
use super::message::{EmbassyMessage, MessageKind};
use super::error::EmbassyError;
use super::constants::NUMBER_OF_MODULES;
use super::surveyor_state::SurveyorState;


#[derive(Debug)]
pub struct StatusManager {
    ecc_status: Vec<ECCStatusResponse>,
    surveyor_status: Vec<SurveyorResponse>
}

impl StatusManager {

    pub fn new() -> Self {
        let eccs = vec![ECCStatusResponse::default(); NUMBER_OF_MODULES as usize];
        let surs = vec![SurveyorResponse::default(); (NUMBER_OF_MODULES-1) as usize];
        return Self { ecc_status: eccs, surveyor_status: surs }
    }

    pub fn reset(&mut self) {
        for eccs in self.ecc_status.iter_mut() {
            *eccs = ECCStatusResponse::default();
        }

        for surs in self.surveyor_status.iter_mut() {
            *surs = SurveyorResponse::default();
        }
    }

    pub fn handle_messages(&mut self, messages: &[EmbassyMessage]) -> Result<(), EmbassyError> {
        for message in messages {
            let module_id = message.id;
            match message.kind {
                MessageKind::ECCOperation => {
                    let resp: ECCOperationResponse = message.try_into()?;
                    if resp.error_code != 0 {
                        tracing::error!("ECC Operation failed with error code {} for module id {}: {}", resp.error_code, module_id, resp.error_message);
                    } else {
                        tracing::info!("ECC Operation completed for module id {}: {}", module_id, resp.text);
                    }
                },
                MessageKind::ECCStatus => {
                    let resp: ECCStatusResponse = message.try_into()?;
                    if resp.error_code != 0 {
                        tracing::error!("ECC Status failed with error code {} for module id {}: {}", resp.error_code, module_id, resp.error_message)
                    }

                    self.ecc_status[module_id as usize] = resp;
                },
                MessageKind::Surveyor => {
                    let resp: SurveyorResponse = message.try_into()?;
                    self.surveyor_status[module_id as usize] = resp;
                }
                _ => {
                    tracing::warn!("Some how recieved a message of kind {} which is not a valid recieving kind!", message.kind);
                }
            }
        }
        Ok(())
    }

    pub fn get_ecc_status(&self) -> &[ECCStatusResponse] {
        &self.ecc_status
    }

    pub fn get_system_ecc_status(&self) -> ECCStatus {
        let sys_status = self.ecc_status[0].state;
        for status in self.ecc_status.iter() {
            if sys_status != status.state {
                return ECCStatus::Inconsistent;
            }
        }
        return ECCStatus::from(sys_status);
    }

    pub fn is_system_ready(&self) -> bool {
        let sys_stat = self.get_system_ecc_status();
        match sys_stat {
            ECCStatus::Ready => true,
            _ => false
        }
    }

    pub fn is_system_running(&self) -> bool {
        let sys_stat = self.get_system_ecc_status();
        match sys_stat {
            ECCStatus::Running => true,
            _ => false
        }
    }

    pub fn get_surveyor_status(&self) -> &[SurveyorResponse] {
        &self.surveyor_status
    }

    pub fn get_surveyor_system_status(&self) -> SurveyorState {
        let sys_status = self.surveyor_status[0].state;
        for status in self.surveyor_status.iter() {
            if sys_status != status.state {
                return SurveyorState::Inconsistent;
            }
        }
        return SurveyorState::from(sys_status);
    }
}