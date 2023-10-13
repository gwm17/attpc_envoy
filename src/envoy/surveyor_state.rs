
const SURVEYOR_ONLINE_STATE_TEXT: &str = "Online";
const SURVEYOR_OFFLINE_STATE_TEXT: &str = "Offline";
const SURVEYOR_INVALID_STATE_TEXT: &str = "Invalid";

#[derive(Debug, Clone)]
pub enum SurveyorState {
    Online,
    Offline,
    Invalid
}

impl From<i32> for SurveyorState {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Offline,
            1 => Self::Online,
            _ => Self::Invalid
        }
    }
}

impl std::fmt::Display for SurveyorState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Online => write!(f, "{SURVEYOR_ONLINE_STATE_TEXT}"),
            Self::Offline => write!(f, "{SURVEYOR_OFFLINE_STATE_TEXT}"),
            Self::Invalid => write!(f, "{SURVEYOR_INVALID_STATE_TEXT}")
        }
    }
}