const MESSAGE_EMPTY_FIELD: &str = "None";

#[derive(Debug, Clone, PartialEq)]
pub enum MessageKind {
    ECC,
    DataRouter,
    Other
}

impl std::fmt::Display for MessageKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ECC => write!(f, "ECC"),
            Self::DataRouter => write!(f, "DataRouter"),
            Self::Other => write!(f, "Other")
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

    pub fn compose_ecc_op(operation: String, id: i32) -> Self {
        EmbassyMessage { kind: MessageKind::ECC, id, operation, response: String::from(MESSAGE_EMPTY_FIELD) }
    }

    pub fn compose_ecc_response(response: String, id: i32) -> Self {
        EmbassyMessage { kind: MessageKind::ECC, id, operation: String::from(MESSAGE_EMPTY_FIELD), response }
    }

}