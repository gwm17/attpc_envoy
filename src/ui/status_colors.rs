use crate::envoy::ecc_operation::ECCStatus;
use crate::envoy::surveyor_state::SurveyorState;
use eframe::egui::Color32;

impl Into<Color32> for &ECCStatus {
    fn into(self) -> Color32 {
        match self {
            ECCStatus::Offline => Color32::GOLD,
            ECCStatus::Busy => Color32::LIGHT_RED,
            ECCStatus::Idle => Color32::WHITE,
            ECCStatus::Described => Color32::BLUE,
            ECCStatus::Prepared => Color32::BLUE,
            ECCStatus::Ready => Color32::LIGHT_GREEN,
            ECCStatus::Running => Color32::GREEN,
            _ => Color32::RED
        }
    }
}

impl Into<Color32> for &SurveyorState {
    fn into(self) -> Color32 {
        match self {
            SurveyorState::Offline => Color32::GOLD,
            SurveyorState::Online => Color32::GREEN,
            _ => Color32::RED
        }
    }
}
// pub fn get_ecc_color(status: &ECCStatus) -> Color32 {
//     match status {
//         ECCStatus::Offline => Color32::GOLD,
//         ECCStatus::Busy => Color32::LIGHT_RED,
//         ECCStatus::Idle => Color32::WHITE,
//         ECCStatus::Described => Color32::BLUE,
//         ECCStatus::Prepared => Color32::BLUE,
//         ECCStatus::Ready => Color32::LIGHT_GREEN,
//         ECCStatus::Running => Color32::GREEN,
//         _ => Color32::RED
//     }
// }

// pub fn get_surveyor_color(status: &SurveyorState) -> Color32 {
//     match status {
//         SurveyorState::Offline => Color32::GOLD,
//         SurveyorState::Online => Color32::GREEN,
//         _ => Color32::RED
//     }
// }