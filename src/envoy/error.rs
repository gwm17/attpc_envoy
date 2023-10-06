use super::message::EmbassyMessage;
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
    MessageParseError(String)
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

impl std::fmt::Display for EnvoyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RequestError(e) => write!(f, "Envoy recieved an error while making a request: {e}"),
            Self::MessageParseError(id) => write!(f, "Envoy {id} failed to parse a message"),
            Self::OperationError(e) => write!(f, "Envoy recieved operation error: {e}"),
            Self::StatusError(e) => write!(f, "Envoy recieved status error: {e}"),
            Self::SendError(e) => write!(f, "Envoy failed to send a message: {e}")
        }
    }
}

impl std::error::Error for EnvoyError {

}

#[derive(Debug)]
pub enum EmbassyError {
    MessageSendError(SendError<EmbassyMessage>),
    MessageRecieveError
}

impl From<SendError<EmbassyMessage>> for EmbassyError {
    fn from(value: SendError<EmbassyMessage>) -> Self {
        Self::MessageSendError(value)
    }
}

impl std::fmt::Display for EmbassyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MessageRecieveError => write!(f, "Embassy had an error while attempting to recieve a message!"),
            Self::MessageSendError(e) => write!(f, "Embassy had an error sending the following message: {e}")
        }
    }
}

impl std::error::Error for EmbassyError {

}