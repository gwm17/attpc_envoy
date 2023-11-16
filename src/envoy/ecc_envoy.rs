use super::message::EmbassyMessage;
use super::ecc_operation::ECCOperation;
use super::error::EnvoyError;
use super::constants::{NUMBER_OF_MODULES, MUTANT_ID, LISTENER_PORT, PROTOCOL, ADDRESS_START};
use reqwest::{Client, Response};
use std::time::Duration;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::sync::broadcast;
use tokio::task::JoinHandle;
use serde::{Deserialize, Serialize};

const ECC_URL_PORT: i32 = 8083;

const ECC_SOAP_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
    <SOAP-ENV:Envelope 
    xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/" 
    xmlns:SOAP-ENC="http://schemas.xmlsoap.org/soap/encoding/"
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xmlns:xsd="http://www.w3.org/2001/XMLSchema"
    xmlns="urn:ecc">
    <SOAP-ENV:Body>
"#;

const ECC_SOAP_FOOTER: &str = r#"
    </SOAP-ENV:Body>
    </SOAP-ENV:Envelope>
"#;

/// Response type for ECC Operations (transitions)
/// Native format is XML
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ECCOperationResponse {
    #[serde(rename="ErrorCode")]
    pub error_code: i32,
    #[serde(rename="ErrorMessage")]
    pub error_message: String,
    #[serde(rename="Text")]
    pub text: String
}

/// Response type for ECC status query
/// Native format is XML
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ECCStatusResponse {
    #[serde(rename="ErrorCode")]
    pub error_code: i32,
    #[serde(rename="ErrorMessage")]
    pub error_message: String,
    #[serde(rename="State")]
    pub state: i32,
    #[serde(rename="Transition")]
    pub transition: i32
}

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
            MUTANT_ID => format!("{ADDRESS_START}.1"),
            _ => format!("{ADDRESS_START}.{}", 60+id)
        };
        let url = Self::url(&address);
        return ECCConfig { id, experiment: experiment.to_string(), address, url};
    }

    fn compose_config_body(&self) -> String {
        let describe = self.describe();
        let prepare = self.experiment.clone();
        let configure = self.experiment.clone();
        format!(r#"<configID>
                        <ConfigId>
                            <SubConfigId type="describe">
                                {describe}
                            </SubConfigId>
                            <SubConfigId type="prepare">
                                {prepare}
                            </SubConfigId>
                            <SubConfigId type="configure">
                                {configure}
                            </SubConfigId>
                        </ConfigId>
                    </configID>"#)
    }

    fn compose_data_link_body(&self) -> String {
        let source = self.source();
        let ip = self.address.clone();
        let router = self.data_router();
        format!(r#"<table>
                        <DataLinkSet>
                            <DataLink>
                                <DataSender id="{source}" />
                                <DataRouter ipAddress="{ip}" name="{router}" port="{LISTENER_PORT}" type="{PROTOCOL}" />
                            </DataLink>
                        </DataLinkSet>
                    </table>"#)
    }

    fn describe(&self) -> String {
        match self.id {
            MUTANT_ID => self.experiment.clone(),
            _ => format!("cobo{}", self.id)
        }
    }

    fn source(&self) -> String {
        match self.id {
            MUTANT_ID => format!("Mutant[master]"),
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

/// # ECCEnvoy
/// The structure encompassing an async task associated with the ECC Server system.
/// ECCEnvoys have two modes, status check and transition. Transition envoys tell the server 
/// when to load/unload configuration data. Status check envoys simply check the status
/// of the server every few seconds.
#[derive(Debug)]
pub struct ECCEnvoy {
    config: ECCConfig,
    connection: Client,
    incoming: mpsc::Receiver<EmbassyMessage>,
    outgoing: mpsc::Sender<EmbassyMessage>,
    cancel: broadcast::Receiver<EmbassyMessage>
}

impl ECCEnvoy {
    pub fn new(config: ECCConfig, rx: mpsc::Receiver<EmbassyMessage>, tx: mpsc::Sender<EmbassyMessage>, cancel: broadcast::Receiver<EmbassyMessage>) -> Result<Self, EnvoyError> {
        //10s default timeouts
        let connection_out = Duration::from_secs(10);
        let req_timeout = Duration::from_secs(10);

        //Probably need some options here, for now just set some timeouts
        let client = Client::builder()
                                .connect_timeout(connection_out)
                                .timeout(req_timeout)
                                .build()?;
        return Ok(Self { config, connection: client, incoming: rx, outgoing: tx, cancel});
    }

    /// This one of the core task loops for an ECCEnvoy. Waits for a
    /// message from the embassy to transition the configuration of 
    /// an ECC Server.
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

    /// This one of the core task loops for an ECCEnvoy. Every two seconds check the
    /// status of the ECC Server. Uses tokio::select! to handle cancelling.
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
                        let response = ECCStatusResponse { error_code: 0, error_message: String::from(""), state: 0, transition: 0 };
                        let message = EmbassyMessage::compose_ecc_response(serde_yaml::to_string(&response)?, self.config.id);
                        self.outgoing.send(message).await?
                    }
                }
            }
        }
    }

    async fn submit_transition(&self, message: EmbassyMessage) -> Result<EmbassyMessage, EnvoyError> {
        let ecc_message = self.compose_ecc_transition_request(message)?;
        let response = self.connection
                                     .post(&self.config.url)
                                     .header("ContentType", "text/xml")
                                     .body(ecc_message)
                                     .send().await?;
        let parsed_response = self.parse_ecc_operation_response(response).await?;
        Ok(parsed_response)
    }

    async fn submit_check_status(&self) -> Result<EmbassyMessage, EnvoyError> {
        let message = format!("{ECC_SOAP_HEADER}<GetStatus>\n</GetStatus>\n{ECC_SOAP_FOOTER}");
        let response = self.connection
                                .post(&self.config.url)
                                .header("ContentType", "text/xml")
                                .body(message)
                                .send().await?;
        let parsed_response = self.parse_ecc_status_response(response).await?;
        Ok(parsed_response)
    }

    async fn parse_ecc_operation_response(&self, response: Response) -> Result<EmbassyMessage, EnvoyError> {
        let text = response.text().await?;
        let mut reader = quick_xml::Reader::from_str(&text);
        let mut parsed = ECCOperationResponse::default();

        reader.read_event()?; //Opening
        reader.read_event()?; //Junk
        reader.read_event()?; //SOAP Decl
        reader.read_event()?; //SOAP Body
        reader.read_event()?; //ECC
        reader.read_event()?; //ErrorCode start tag
        let event = reader.read_event()?; //ErrorCode payload
        parsed.error_code = match event {
            quick_xml::events::Event::Text(t) => String::from_utf8(t.to_vec())?.parse()?,
            _ => {
                return Err(EnvoyError::XMLConversionError)
            }
        };
        reader.read_event()?; //ErrorCode end tag
        reader.read_event()?; //ErrorMesage start tag
        let event = reader.read_event()?; //ErrorMessage payload or end tag
        let mut is_msg = true;
        parsed.error_message = match event {
            quick_xml::events::Event::Text(t) => String::from_utf8(t.to_vec())?,
            _ => {
                is_msg = false;
                String::from("")
            }
        };
        if is_msg {
            reader.read_event()?; //ErrorMessage end tag
        }
        reader.read_event()?; //Text start tag
        let event = reader.read_event()?; //Text payload
        parsed.text = match event {
            quick_xml::events::Event::Text(t) => String::from_utf8(t.to_vec())?,
            _ => String::from("")
        };

        Ok(EmbassyMessage::compose_ecc_response(serde_yaml::to_string(&parsed)?, self.config.id))
    }

    async fn parse_ecc_status_response(&self, response: Response) -> Result<EmbassyMessage, EnvoyError> {
        let text = response.text().await?;
        let mut reader = quick_xml::Reader::from_str(&text);
        let mut parsed: ECCStatusResponse = ECCStatusResponse::default();

        reader.read_event()?; //Opening
        reader.read_event()?; //Junk
        reader.read_event()?; //SOAP Decl
        reader.read_event()?; //SOAP Body
        reader.read_event()?; //ECC
        reader.read_event()?; //ErrorCode start tag
        let event = reader.read_event()?; //ErrorCode payload
        parsed.error_code = match event {
            quick_xml::events::Event::Text(t) => String::from_utf8(t.to_vec())?.parse()?,
            _ => {
                return Err(EnvoyError::XMLConversionError)
            }
        };
        reader.read_event()?; //ErrorCode end tag
        reader.read_event()?; //ErrorMesage start tag
        let event = reader.read_event()?; //ErrorMessage payload or end tag
        let mut is_msg = true;
        parsed.error_message = match event {
            quick_xml::events::Event::Text(t) => String::from_utf8(t.to_vec())?,
            _ => {
                is_msg = false;
                String::from("")
            }
        };
        if is_msg {
            reader.read_event()?; //ErrorMessage end tag
        }
        reader.read_event()?; //State start tag
        let event = reader.read_event()?; //State payload
        parsed.state = match event {
            quick_xml::events::Event::Text(t) => String::from_utf8(t.to_vec())?.parse()?,
            _ => return Err(EnvoyError::XMLConversionError)
        };
        reader.read_event()?; //State end tag
        reader.read_event()?; //Transition start tag
        let event = reader.read_event()?; //Transition payload
        parsed.transition = match event {
            quick_xml::events::Event::Text(t) => String::from_utf8(t.to_vec())?.parse()?,
            _ => return Err(EnvoyError::XMLConversionError)
        };

        let status_response = EmbassyMessage::compose_ecc_status(serde_yaml::to_string(&parsed)?, self.config.id);
        Ok(status_response)
    }

    fn compose_ecc_transition_request(&self, message: EmbassyMessage) -> Result<String, EnvoyError> {
        let op = ECCOperation::try_from(message.operation)?;
        let config = self.config.compose_config_body();
        let link = self.config.compose_data_link_body();
        return Ok(format!("{ECC_SOAP_HEADER}<{op}>\n{config}{link}</{op}>\n{ECC_SOAP_FOOTER}"));
    }

}

/// Startup the ECC communication system
/// Takes in a runtime, experiment name, and a channel to send data to the embassy. Spawns the ECCEnvoys with tasks to either wait for
/// a command to transition that ECC DAQ or to periodically check the status of that particular ECC DAQ.
pub fn startup_ecc_envoys(runtime: &mut tokio::runtime::Runtime, experiment: &str, ecc_tx: &mpsc::Sender<EmbassyMessage>, cancel: &broadcast::Sender<EmbassyMessage>) -> (Vec<JoinHandle<()>>, HashMap<i32, mpsc::Sender<EmbassyMessage>>) {
    let mut transition_switchboard = HashMap::new();
    let mut handles: Vec<JoinHandle<()>> = vec![];

    //spin up the transition envoys
    for id in 0..NUMBER_OF_MODULES {
        let config = ECCConfig::new(id, experiment);
        let (embassy_tx, ecc_rx) = mpsc::channel::<EmbassyMessage>(10);
        let this_ecc_tx = ecc_tx.clone();
        let this_cancel = cancel.subscribe();
        let handle = runtime.spawn(async move {
            match ECCEnvoy::new(config, ecc_rx, this_ecc_tx, this_cancel) {
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
    for id in 0..NUMBER_OF_MODULES {
        let config = ECCConfig::new(id, experiment);
        //The incoming channel is unused in the status envoy, however this may be changed later.
        //Could be useful to tie the update rate to the GUI?
        let (_, ecc_rx) = mpsc::channel::<EmbassyMessage>(10);
        let this_ecc_tx = ecc_tx.clone();
        let this_cancel = cancel.subscribe();
        let handle = runtime.spawn(async move {
            match ECCEnvoy::new(config, ecc_rx, this_ecc_tx, this_cancel) {
                Ok(mut ev) => {
                    match ev.wait_check_status().await {
                        Ok(()) =>(),
                        Err(e) => tracing::error!("ECC status envoy ran into an error: {}", e)
                    }
                }
                Err(e) => tracing::error!("Error creating ECC status envoy: {}", e)
            }
        });

        handles.push(handle);
    }

    return (handles, transition_switchboard);
}