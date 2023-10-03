
#[derive(Debug, Clone)]
pub enum ECCStatus {
    Disconnected,
    Idle,
    Prepared,
    Described,
    Ready,
    Active
}

impl std::fmt::Display for ECCStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Disconnected => write!(f, "Disconnected"),
            Self::Idle => write!(f, "Idle"),
            Self::Prepared => write!(f, "Prepared"),
            Self::Described => write!(f, "Described"),
            Self::Ready => write!(f, "Ready"),
            Self::Active => write!(f, "Active")
        }
    }
}

#[derive(Debug, Clone)]
pub struct ECCMessage {
    pub status: ECCStatus,
    pub id: String,
    pub body: String
}

impl Default for ECCMessage {
    fn default() -> Self {
        Self { status: ECCStatus::Disconnected, id: String::from("Invalid"), body: String::new() }
    }
}

impl std::fmt::Display for ECCMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ECCMessage from {} with status {}: {}", self.id, self.status, self.body)
    }
}

#[derive(Debug, Clone)]
pub struct EmbassyMessage {
    kind: String,
    id: String,
    body: String
}

impl From<ECCMessage> for EmbassyMessage {
    fn from(value: ECCMessage) -> Self {
        Self { kind: String::from("ECC"), id: value.id, body: value.body }
    }
}