use super::message::EmbassyMessage;
use super::ecc_operation::{ECCOperation, ECCStatus};
use super::error::EnvoyError;
use reqwest::{Client, Response};
use std::time::Duration;
use tokio::sync::mpsc::{Receiver, Sender};

const NUMBER_OF_ECC_ENVOYS: usize = 11;

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
            ECC_MUTANT_ID => self.experiment,
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
        format!("http//{}:{}", address, ECC_URL_PORT)
    }
}

#[derive(Debug)]
pub struct ECCEnvoy {
    status: ECCStatus,
    config: ECCConfig,
    connection: Client,
    incoming: Receiver<EmbassyMessage>,
    outgoing: Sender<EmbassyMessage>
}

impl ECCEnvoy {
    pub async fn new(config: ECCConfig, rx: Receiver<EmbassyMessage>, tx: Sender<EmbassyMessage>) -> Result<Self, EnvoyError> {
        //3min default timeouts
        let connection_out = Duration::from_secs(360);
        let req_timeout = Duration::from_secs(360);

        //Probably need some options here, for now just set some timeouts
        let client = Client::builder()
                                .connect_timeout(connection_out)
                                .timeout(req_timeout)
                                .build()?;
        let mut envoy = Self { status: ECCStatus::Offline, config, connection: client, incoming: rx, outgoing: tx };


        //Send a get request to establish the connection and retrieve the status
        let response: Response = envoy.connection.get(&envoy.config.url)
                                                 .send().await?;
        let message = envoy.parse_ecc_response(response)?;
        envoy.status = ECCStatus::try_from(message.response)?;
        Ok(envoy)
    }

    pub async fn wait_for_transition(&mut self) -> Result<(), EnvoyError> {
        loop {
            if let Some(message) = self.incoming.recv().await {
                let response = self.submit_transition().await?;
                self.outgoing.send(response).await?;
            } else {
                break;
            }
        }
        Ok(())
    }

    pub async fn wait_check_status(&self) -> Result<(), EnvoyError> {
        loop {
            if let Ok(response) = self.submit_check_status().await {
                self.outgoing.send(response).await?
            } else {
                let message = EmbassyMessage::compose_ecc_response(String::from("Offline"), self.config.id);
                self.outgoing.send(message).await?
            }
        }
    }

    async fn submit_transition(&self) -> Result<EmbassyMessage, EnvoyError> {
        let response = self.connection
                                     .post(&self.config.url)
                                     .header("ContentType", "text/xml")
                                     .send().await?;
        let message = self.parse_ecc_response(response)?;
        Ok(message)
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

}