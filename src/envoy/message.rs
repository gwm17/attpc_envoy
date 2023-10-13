use super::ecc_envoy::{ECCOperationResponse, ECCStatusResponse};
use super::error::EmbassyError;
use super::surveyor_envoy::SurveyorResponse;

const MESSAGE_EMPTY_FIELD: &str = "None";

#[derive(Debug, Clone, PartialEq)]
pub enum MessageKind {
    ECCOperation,
    ECCStatus,
    Surveyor,
    Other,
    Cancel
}

impl std::fmt::Display for MessageKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ECCOperation => write!(f, "ECCOperation"),
            Self::ECCStatus => write!(f, "ECCStatus"),
            Self::Surveyor => write!(f, "Surveyor"),
            Self::Other => write!(f, "Other"),
            Self::Cancel => write!(f, "Cancel"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EmbassyMessage {
    pub kind: MessageKind,
    pub id: i32,
    pub operation: String,
    pub response: String
}

impl std::fmt::Display for EmbassyMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EmbassyMessage from {} of kind {} for operation {} with response: {}", self.id, self.kind, self.operation, self.response)
    }
}

impl EmbassyMessage {

    pub fn compose_surveyor_response(response: String, id: i32) -> Self {
        EmbassyMessage { kind: MessageKind::Surveyor, id, operation: String::from(MESSAGE_EMPTY_FIELD), response }
    }

    pub fn compose_ecc_op(operation: String, id: i32) -> Self {
        EmbassyMessage { kind: MessageKind::ECCOperation, id, operation, response: String::from(MESSAGE_EMPTY_FIELD) }
    }

    pub fn compose_ecc_response(response: String, id: i32) -> Self {
        EmbassyMessage { kind: MessageKind::ECCOperation, id, operation: String::from(MESSAGE_EMPTY_FIELD), response }
    }

    pub fn compose_ecc_status(response: String, id: i32) -> Self {
        EmbassyMessage { kind: MessageKind::ECCStatus, id, operation: String::from(MESSAGE_EMPTY_FIELD), response }
    }

    pub fn compose_cancel() -> Self {
        EmbassyMessage { kind: MessageKind::Other, id: 0, operation: String::from(MESSAGE_EMPTY_FIELD), response: String::from(MESSAGE_EMPTY_FIELD) }
    }

}

impl TryInto<ECCStatusResponse> for EmbassyMessage {
    type Error = EmbassyError;
    fn try_into(self) -> Result<ECCStatusResponse, Self::Error> {
        match self.kind {
            MessageKind::ECCStatus => Ok(serde_yaml::from_str::<ECCStatusResponse>(&self.response)?),
            _ => Err(Self::Error::MessageKindError(MessageKind::ECCStatus, self.kind))
        }
    }
}

impl TryInto<ECCOperationResponse> for EmbassyMessage {
    type Error = EmbassyError;
    fn try_into(self) -> Result<ECCOperationResponse, Self::Error> {
        match self.kind {
            MessageKind::ECCOperation => Ok(serde_yaml::from_str::<ECCOperationResponse>(&self.response)?),
            _ => Err(Self::Error::MessageKindError(MessageKind::ECCOperation, self.kind))
        }
    }
}

impl TryInto<SurveyorResponse> for EmbassyMessage {
    type Error = EmbassyError;
    fn try_into(self) -> Result<SurveyorResponse, Self::Error> {
        match self.kind {
            MessageKind::Surveyor => Ok(serde_yaml::from_str::<SurveyorResponse>(&self.response)?),
            _ => Err(Self::Error::MessageKindError(MessageKind::Surveyor, self.kind))
        }
    }
}