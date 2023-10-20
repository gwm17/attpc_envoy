use serde::{Deserialize, Serialize};


/// # Config
/// (De)Serializable application configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub experiment: String,
    pub run_number: i32,
    pub description: String
}

impl Config {
    pub fn new() -> Self {
        return Config { experiment: String::from("Exp"), run_number: 0, description: String::from("Write here")}
    }
}