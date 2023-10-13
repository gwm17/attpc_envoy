use super::message::EmbassyMessage;
use super::error::EnvoyError;
use super::constants::{NUMBER_OF_MODULES, ADDRESS_START};
use reqwest::{Client, Response};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::broadcast;
use tokio::task::JoinHandle;
use serde::{Deserialize, Serialize};

const SURVEYOR_URL_PORT: i32 = 8081;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SurveyorResponse {
    pub state: i32,
    pub location: String,
    pub disk_status: String,
    pub percent_used: String,
    pub disk_space: u64,
    pub files: i32,
    pub bytes_used: u64,
    pub data_rate: f64,
}

impl Default for SurveyorResponse {
    fn default() -> Self {
        Self { state: 0, location: String::from("N/A"), disk_status: String::from("N/A"), percent_used: String::from("N/A"), disk_space: 0, files: 0, bytes_used: 0, data_rate: 0.0 }
    }
}


#[derive(Debug, Clone)]
pub struct SurveyorConfig {
    id: i32,
    address: String,
    url: String
}

impl SurveyorConfig {
    pub fn new(id: i32) -> Self {
        let address = Self::address(&id);
        let url = Self::url(&address);

        Self { id, address, url }
    }

    fn address(id: &i32) -> String {
        format!("{ADDRESS_START}.{}", 60+id)
    }

    fn url(address: &str) -> String {
        format!("http://{address}:{SURVEYOR_URL_PORT}")
    }
}

#[derive(Debug)]
pub struct SurveyorEnvoy {
    config: SurveyorConfig,
    connection: Client,
    outgoing: mpsc::Sender<EmbassyMessage>,
    cancel: broadcast::Receiver<EmbassyMessage>,
    last_bytes: u64
}

impl SurveyorEnvoy {

    pub fn new(config: SurveyorConfig, tx: mpsc::Sender<EmbassyMessage>, cancel: broadcast::Receiver<EmbassyMessage>) -> Result<Self, EnvoyError> {
        //3min default timeouts
        let connection_out = Duration::from_secs(10);
        let req_timeout = Duration::from_secs(10);

        //Probably need some options here, for now just set some timeouts
        let client = Client::builder()
                                .connect_timeout(connection_out)
                                .timeout(req_timeout)
                                .build()?;
        return Ok(Self { config, connection: client, outgoing: tx, cancel, last_bytes: 0});
    }

    pub async fn wait_check_status(&mut self) -> Result<(), EnvoyError> {
        loop {
            tokio::select! {
                _ = self.cancel.recv() => {
                    return Ok(());
                }

                _ = tokio::time::sleep(Duration::from_secs(2)) => {
                    if let Ok(response) = self.submit_check_status().await {
                        self.outgoing.send(response).await?
                    } else {
                        let message = EmbassyMessage::compose_surveyor_response(serde_yaml::to_string(&SurveyorResponse::default())?, self.config.id);
                        self.outgoing.send(message).await?
                    }
                }
            }
        }
    }

    async fn submit_check_status(&mut self) -> Result<EmbassyMessage, EnvoyError> {
        let response = self.connection
                                .get(&self.config.url)
                                .send().await?;
        let parsed_response = self.parse_response(response).await?;
        Ok(parsed_response)
    }

    async fn parse_response(&mut self, response: Response) -> Result<EmbassyMessage, EnvoyError> {
        let response_text = response.text().await?;
        let mut status = SurveyorResponse::default();
        let lines: Vec<&str> = response_text.lines().collect();

        if lines.len() == 0 {
            return Ok(EmbassyMessage::compose_surveyor_response(serde_yaml::to_string(&status)?, self.config.id));
        }

        status.state = lines[0].parse::<i32>()?;
        if status.state == 0 {
            return Ok(EmbassyMessage::compose_surveyor_response(serde_yaml::to_string(&status)?, self.config.id))
        }
        status.location = String::from(lines[1]);
        let line_entries: Vec<&str> = lines[3].split_whitespace().collect();
        status.percent_used = String::from(line_entries[4]);
        status.disk_space = line_entries[1].parse::<u64>()? * 512;

        let mut bytes: u64 = 0;
        let mut n_files = 0;
        for line in lines[4..].iter() {
            if line.contains("graw") {
                let line_entries: Vec<&str> = line.split_whitespace().collect();
                bytes += line_entries.last().unwrap().parse::<u64>()?;
                n_files += 1;
            }
        }

        if n_files > 0 {
            status.disk_status = String::from("Filled");
        } else {
            status.disk_status = String::from("Empty");
        }
        
        status.files = n_files;
        status.bytes_used = bytes;

        status.data_rate = ((bytes - self.last_bytes) as f64) / 2.0;

        self.last_bytes = bytes;

        Ok(EmbassyMessage::compose_surveyor_response(serde_yaml::to_string(&status)?, self.config.id))
    }

}

pub fn startup_surveyor_envoys(runtime: &mut tokio::runtime::Runtime, surveyor_tx: &mpsc::Sender<EmbassyMessage>, cancel: &broadcast::Sender<EmbassyMessage>) -> Vec<JoinHandle<()>> {
    let mut handles: Vec<JoinHandle<()>> = vec![];

    //spin up the surveyor envoys, Mutant does not get a data router/surveyor
    for id in 0..(NUMBER_OF_MODULES-1) {
        let config = SurveyorConfig::new(id);
        let this_surveyor_tx = surveyor_tx.clone();
        let this_cancel = cancel.subscribe();
        let handle = runtime.spawn(async move {
            match SurveyorEnvoy::new(config, this_surveyor_tx, this_cancel) {
                Ok(mut ev) => {
                    match ev.wait_check_status().await {
                        Ok(()) =>(),
                        Err(e) => tracing::error!("Surveyor status envoy ran into an error: {}", e)
                    }
                }
                Err(e) => tracing::error!("Error creating Surveyor status envoy: {}", e)
            }
        });

        handles.push(handle);
    }

    return handles;
}