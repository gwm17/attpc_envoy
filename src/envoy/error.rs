
#[derive(Debug)]
pub enum EnvoyError {
    RequestError(reqwest::Error),
    MessageParseError(String)
}

impl From<reqwest::Error> for EnvoyError {
    fn from(value: reqwest::Error) -> Self {
        Self::RequestError(value)
    }
}

impl std::fmt::Display for EnvoyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RequestError(e) => write!(f, "Envoy recieved an error while making a request: {e}"),
            Self::MessageParseError(id) => write!(f, "Envoy {id} failed to parse a message")
        }
    }
}