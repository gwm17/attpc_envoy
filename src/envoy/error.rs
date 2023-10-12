use super::message::{EmbassyMessage, MessageKind};
use tokio::sync::mpsc::error::SendError;

#[derive(Debug)]
pub enum ECCOperationError {
    BadString(String)
}

impl std::fmt::Display for ECCOperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadString(s) => write!(f, "Could not convert string {s} to ECCOperation!")
        }
    }
}

impl std::error::Error for ECCOperationError {
    
}

#[derive(Debug)]
pub enum ECCStatusError {
    BadString(String)
}

impl std::fmt::Display for ECCStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadString(s) => write!(f, "Could not convert string {s} to ECCStatus!")
        }
    }
}

impl std::error::Error for ECCStatusError {
    
}

#[derive(Debug)]
pub enum EnvoyError {
    RequestError(reqwest::Error),
    SendError(SendError<EmbassyMessage>),
    StatusError(ECCStatusError),
    OperationError(ECCOperationError),
    MessageParseError(serde_yaml::Error),
    StringToIntError(std::num::ParseIntError),
    StringToFloatError(std::num::ParseFloatError),
}

impl From<reqwest::Error> for EnvoyError {
    fn from(value: reqwest::Error) -> Self {
        Self::RequestError(value)
    }
}

impl From<SendError<EmbassyMessage>> for EnvoyError {
    fn from(value: SendError<EmbassyMessage>) -> Self {
        Self::SendError(value)
    }
}

impl From<ECCStatusError> for EnvoyError {
    fn from(value: ECCStatusError) -> Self {
        Self::StatusError(value)
    }
}

impl From<ECCOperationError> for EnvoyError {
    fn from(value: ECCOperationError) -> Self {
        Self::OperationError(value)
    }
}

impl From<serde_yaml::Error> for EnvoyError {
    fn from(value: serde_yaml::Error) -> Self {
        Self::MessageParseError(value)
    }
}

impl From<std::num::ParseIntError> for EnvoyError {
    fn from(value: std::num::ParseIntError) -> Self {
        Self::StringToIntError(value)
    }
}

impl From<std::num::ParseFloatError> for EnvoyError {
    fn from(value: std::num::ParseFloatError) -> Self {
        Self::StringToFloatError(value)
    }
}

impl std::fmt::Display for EnvoyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RequestError(e) => write!(f, "Envoy recieved an error while making a request: {e}"),
            Self::MessageParseError(e) => write!(f, "Envoy failed to parse a message to yaml: {e}"),
            Self::OperationError(e) => write!(f, "Envoy recieved operation error: {e}"),
            Self::StatusError(e) => write!(f, "Envoy recieved status error: {e}"),
            Self::SendError(e) => write!(f, "Envoy failed to send a message: {e}"),
            Self::StringToIntError(e) => write!(f, "Envoy failed to parse string to integer: {e}"),
            Self::StringToFloatError(e) => write!(f, "Envoy failed to parse string to float: {e}")
        }
    }
}

impl std::error::Error for EnvoyError {

}

#[derive(Debug)]
pub enum EmbassyError {
    MessageSendError(SendError<EmbassyMessage>),
    MessageKindError(MessageKind, MessageKind),
    MessageParseError(serde_yaml::Error),
    MessageRecieveError
}

impl From<SendError<EmbassyMessage>> for EmbassyError {
    fn from(value: SendError<EmbassyMessage>) -> Self {
        Self::MessageSendError(value)
    }
}

impl From<serde_yaml::Error> for EmbassyError {
    fn from(value: serde_yaml::Error) -> Self {
        Self::MessageParseError(value)
    }
}

impl std::fmt::Display for EmbassyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MessageKindError(expected, recieved) => write!(f, "Embassy expected {expected} message, recieved {recieved} message!"),
            Self::MessageSendError(e) => write!(f, "Embassy had an error sending the following message: {e}"),
            Self::MessageParseError(e) => write!(f, "Embassy had an error parsing a message: {e}"),
            Self::MessageRecieveError => write!(f, "Embassy communication lines were disconnected!")
        }
    }
}

impl std::error::Error for EmbassyError {

}