use super::message::{EmbassyMessage, ECCMessage};
use tokio::sync::mpsc::error::SendError;

#[derive(Debug)]
pub enum EnvoyError {
    RequestError(reqwest::Error),
    SendError(SendError<ECCMessage>),
    MessageParseError(String)
}

impl From<reqwest::Error> for EnvoyError {
    fn from(value: reqwest::Error) -> Self {
        Self::RequestError(value)
    }
}

impl From<SendError<ECCMessage>> for EnvoyError {
    fn from(value: SendError<ECCMessage>) -> Self {
        Self::SendError(value)
    }
}

impl std::fmt::Display for EnvoyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RequestError(e) => write!(f, "Envoy recieved an error while making a request: {e}"),
            Self::MessageParseError(id) => write!(f, "Envoy {id} failed to parse a message"),
            Self::SendError(e) => write!(f, "Envoy failed to send a message: {e}")
        }
    }
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