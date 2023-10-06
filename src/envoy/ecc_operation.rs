use super::error::{ECCOperationError, ECCStatusError};

const ECC_OFFLINE_STATUS: &str = "Offline";
const ECC_TRANSITION_STATUS: &str = "Transition";
const ECC_IDLE_STATUS: &str = "Idle";
const ECC_PREPARED_STATUS: &str = "Prepared";
const ECC_DESCRIBED_STATUS: &str = "Described";
const ECC_READY_STATUS: &str = "Ready";
const ECC_RUNNING_STATUS: &str = "Running";
const ECC_DESCRIBE_OP: &str = "Describe";
const ECC_PREPARE_OP: &str = "Prepare";
const ECC_CONFIGURE_OP: &str = "Configure";
const ECC_START_OP: &str = "Start";
const ECC_UNDO_OP: &str = "Undo";
const ECC_BREAKUP_OP: &str = "Breakup";
const ECC_STOP_OP: &str = "Stop";
const ECC_CHECK_OP: &str = "Check";

#[derive(Debug, Clone)]
pub enum ECCStatus {
    Offline,
    Transition,
    Idle,
    Prepared,
    Described,
    Ready,
    Running
}

impl std::fmt::Display for ECCStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Offline => write!(f, "{ECC_OFFLINE_STATUS}"),
            Self::Transition => write!(f, "{ECC_TRANSITION_STATUS}"),
            Self::Idle => write!(f, "{ECC_IDLE_STATUS}"),
            Self::Prepared => write!(f, "{ECC_PREPARED_STATUS}"),
            Self::Described => write!(f, "{ECC_DESCRIBED_STATUS}"),
            Self::Ready => write!(f, "{ECC_READY_STATUS}"),
            Self::Running => write!(f, "{ECC_RUNNING_STATUS}")
        }
    }
}

impl Into<String> for ECCStatus {
    fn into(self) -> String {
        String::from(match self {
            Self::Offline => ECC_OFFLINE_STATUS,
            Self::Transition => ECC_TRANSITION_STATUS,
            Self::Idle => ECC_IDLE_STATUS,
            Self::Prepared => ECC_PREPARED_STATUS,
            Self::Described => ECC_DESCRIBED_STATUS,
            Self::Ready => ECC_READY_STATUS,
            Self::Running => ECC_RUNNING_STATUS
        })
    }
}

impl TryFrom<String> for ECCStatus {
    type Error = ECCStatusError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            ECC_OFFLINE_STATUS => Ok(Self::Offline),
            ECC_TRANSITION_STATUS => Ok(Self::Transition),
            ECC_IDLE_STATUS => Ok(Self::Idle),
            ECC_PREPARED_STATUS => Ok(Self::Prepared),
            ECC_DESCRIBED_STATUS => Ok(Self::Described),
            ECC_READY_STATUS => Ok(Self::Ready),
            ECC_RUNNING_STATUS => Ok(Self::Running),
            _ => Err(Self::Error::BadString(value))
        }
    }
}

#[derive(Debug, Clone)]
pub enum ECCOperation {
    Describe,
    Prepare,
    Configure,
    Start,
    Undo,
    Breakup,
    Stop,
    CheckStatus
}

impl std::fmt::Display for ECCOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Describe => write!(f, "{ECC_DESCRIBE_OP}"),
            Self::Prepare => write!(f, "{ECC_PREPARE_OP}"),
            Self::Configure => write!(f, "{ECC_CONFIGURE_OP}"),
            Self::Start => write!(f, "{ECC_START_OP}"),
            Self::Undo => write!(f, "{ECC_UNDO_OP}"),
            Self::Breakup => write!(f, "{ECC_BREAKUP_OP}"),
            Self::Stop => write!(f, "{ECC_STOP_OP}"),
            Self::CheckStatus => write!(f, "{ECC_CHECK_OP}")
        }
    }
}

impl TryFrom<String> for ECCOperation {
    type Error = ECCOperationError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            ECC_DESCRIBE_OP => Ok(Self::Describe),
            ECC_PREPARE_OP => Ok(Self::Prepare),
            ECC_CONFIGURE_OP => Ok(Self::Configure),
            ECC_START_OP => Ok(Self::Start),
            ECC_UNDO_OP => Ok(Self::Undo),
            ECC_BREAKUP_OP => Ok(Self::Breakup),
            ECC_STOP_OP => Ok(Self::Stop),
            ECC_CHECK_OP => Ok(Self::CheckStatus),
            _ => Err(Self::Error::BadString(value))
        }
    }
}

impl Into<String> for ECCOperation {
    fn into(self) -> String {
        String::from( match self {
            Self::Describe => ECC_DESCRIBE_OP,
            Self::Prepare => ECC_PREPARE_OP,
            Self::Configure => ECC_CONFIGURE_OP,
            Self::Start => ECC_START_OP,
            Self::Undo => ECC_UNDO_OP,
            Self::Breakup => ECC_BREAKUP_OP,
            Self::Stop => ECC_STOP_OP,
            Self::CheckStatus => ECC_CHECK_OP
        })
    }
}