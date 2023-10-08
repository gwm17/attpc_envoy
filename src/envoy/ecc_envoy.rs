use super::message::EmbassyMessage;
use super::ecc_operation::ECCOperation;
use super::error::EnvoyError;
use reqwest::{Client, Response};
use std::time::Duration;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::sync::broadcast;
use tokio::task::JoinHandle;

const NUMBER_OF_ECC_ENVOYS: i32 = 11;

const ECC_MUTANT_ID: i32 = 11;

const ECC_COMMAND_PORT: i32 = 46005;
const ECC_URL_PORT: i32 = 8083;

const ECC_PROTOCOL: &str = "TCP";

const ECC_SOAP_HEADER: &str = "\
    <?xml version=\"1.0\" encoding=\"UTF-8\"?> \n
    <SOAP-ENV:Envelope\n\
    xmlns:SOAP-ENV=\"http://schemas.xmlsoap.org/soap/envelope/\"\n\
    xmlns:SOAP-ENC=\"http://schemas.xmlsoap.org/soap/encoding/\"\n\
    xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\"\n\
    xmlns:xsd=\"http://www.w3.org/2001/XMLSchema\"\n\
    xmlns=\"urn:ecc\">\n\
    <SOAP-ENV:Body>\n
";

const ECC_SOAP_FOOTER: &str = "\
    </SOAP-ENV:Body>\n\
    </SOAP-ENV:Envelope>\n
";

#[derive(Debug, Clone)]
pub struct ECCConfig {
    id: i32,
    experiment: String,
    address: String,
    url: String
}

impl ECCConfig {
    pub fn new(id: i32, experiment: &str) -> ECCConfig {
        let address = match id {
            ECC_MUTANT_ID => String::from("192.168.41.1"),
            _ => format!("192.168.41.{}", 60+id)
        };
        let url = Self::url(&address);
        return ECCConfig { id, experiment: experiment.to_string(), address, url};
    }

    fn describe(&self) -> String {
        match self.id {
            ECC_MUTANT_ID => self.experiment.clone(),
            _ => format!("cobo{}", self.id)
        }
    }

    fn source(&self) -> String {
        match self.id {
            ECC_MUTANT_ID => format!("Mutant[master]"),
            _ => format!("CoBo[{}]", self.id)
        }
    }

    fn data_router(&self) -> String {
        format!("data{}", self.id)
    }

    fn url(address: &str) -> String {
        format!("http://{}:{}", address, ECC_URL_PORT)
    }
}

#[derive(Debug)]
pub struct ECCEnvoy {
    config: ECCConfig,
    connection: Client,
    incoming: mpsc::Receiver<EmbassyMessage>,
    outgoing: mpsc::Sender<EmbassyMessage>,
    cancel: broadcast::Receiver<EmbassyMessage>
}

impl ECCEnvoy {
    pub async fn new(config: ECCConfig, rx: mpsc::Receiver<EmbassyMessage>, tx: mpsc::Sender<EmbassyMessage>, cancel: broadcast::Receiver<EmbassyMessage>) -> Result<Self, EnvoyError> {
        //3min default timeouts
        let connection_out = Duration::from_secs(10);
        let req_timeout = Duration::from_secs(10);

        //Probably need some options here, for now just set some timeouts
        let client = Client::builder()
                                .connect_timeout(connection_out)
                                .timeout(req_timeout)
                                .build()?;
        return Ok(Self { config, connection: client, incoming: rx, outgoing: tx, cancel});
    }

    pub async fn wait_for_transition(&mut self) -> Result<(), EnvoyError> {
        loop {
            tokio::select! {
                _ = self.cancel.recv() => {
                    return Ok(())
                }

                data = self.incoming.recv() => {
                    if let Some(message) = data {
                        let response = self.submit_transition(message).await?;
                        self.outgoing.send(response).await?;
                    } else {
                        return Ok(())
                    }
                }
            }
        }
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
                        let message = EmbassyMessage::compose_ecc_response(String::from("Offline"), self.config.id);
                        self.outgoing.send(message).await?
                    }
                }
            }
        }
    }

    async fn submit_transition(&self, message: EmbassyMessage) -> Result<EmbassyMessage, EnvoyError> {
        let response = self.connection
                                     .post(&self.config.url)
                                     .header("ContentType", "text/xml")
                                     .send().await?;
        let parsed_response = self.parse_ecc_response(response)?;
        Ok(parsed_response)
    }

    async fn submit_check_status(&self) -> Result<EmbassyMessage, EnvoyError> {
        todo!()
    }

    fn parse_ecc_response(&self, response: Response) -> Result<EmbassyMessage, EnvoyError> {
        todo!()
    }

    fn compose_ecc_transition_request(&self, message: EmbassyMessage) -> Result<String, EnvoyError> {
        let op = ECCOperation::try_from(message.operation)?;
        return Ok(format!("{ECC_SOAP_HEADER}<{op}>\n</{op}>\n{ECC_SOAP_FOOTER}"));
    }

    pub fn get_id(&self) -> i32 {
        self.config.id
    }

}

/// Startup the ECC communication system
/// Takes in a runtime, experiment name, and a channel to send data to the embassy. Spawns the ECCEnvoys with tasks to either wait for
/// a command to transition that ECC DAQ or to periodically check the status of that particular ECC DAQ.
pub fn startup_ecc_envoys(runtime: &mut tokio::runtime::Runtime, experiment: &str, ecc_tx: &mpsc::Sender<EmbassyMessage>, cancel: &broadcast::Sender<EmbassyMessage>) -> (Vec<JoinHandle<()>>, HashMap<i32, mpsc::Sender<EmbassyMessage>>) {
    let mut transition_switchboard = HashMap::new();
    let mut handles: Vec<JoinHandle<()>> = vec![];

    //spin up the transition envoys
    for id in 0..NUMBER_OF_ECC_ENVOYS {
        let config = ECCConfig::new(id, experiment);
        let (embassy_tx, ecc_rx) = mpsc::channel::<EmbassyMessage>(10);
        let this_ecc_tx = ecc_tx.clone();
        let this_cancel = cancel.subscribe();
        let handle = runtime.spawn(async move {
            match ECCEnvoy::new(config, ecc_rx, this_ecc_tx, this_cancel).await {
                Ok(mut ev) => {
                    match ev.wait_for_transition().await {
                        Ok(()) =>(),
                        Err(e) => tracing::error!("ECC transition envoy ran into an error: {}", e)
                    }
                }
                Err(e) => tracing::error!("Error creating ECC transition envoy: {}", e)
            }
        });

        transition_switchboard.insert(id, embassy_tx);
        handles.push(handle);
    }
    
    //spin up the status envoys
    for id in 0..NUMBER_OF_ECC_ENVOYS {
        let config = ECCConfig::new(id, experiment);
        //The incoming channel is unused in the status envoy, however this may be changed later.
        //Could be useful to tie the update rate to the GUI?
        let (_, ecc_rx) = mpsc::channel::<EmbassyMessage>(10);
        let this_ecc_tx = ecc_tx.clone();
        let this_cancel = cancel.subscribe();
        let handle = runtime.spawn(async move {
            match ECCEnvoy::new(config, ecc_rx, this_ecc_tx, this_cancel).await {
                Ok(mut ev) => {
                    match ev.wait_check_status().await {
                        Ok(()) =>(),
                        Err(e) => tracing::error!("ECC transition envoy ran into an error: {}", e)
                    }
                }
                Err(e) => tracing::error!("Error creating ECC transition envoy: {}", e)
            }
        });

        handles.push(handle);
    }

    return (handles, transition_switchboard);
}