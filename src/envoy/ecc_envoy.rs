use super::message::{ECCStatus, ECCMessage};
use super::error::EnvoyError;
use reqwest::{Client, Response};
use std::time::Duration;

#[derive(Debug)]
pub struct ECCEnvoy {
    status: ECCStatus,
    id: String,
    connection: Client
}

impl ECCEnvoy {
    pub async fn new(id: String) -> Result<Self, EnvoyError> {
        //3min default timeouts
        let connection_out = Duration::from_secs(360);
        let req_timeout = Duration::from_secs(360);

        //Probably need some options here, for now just set some timeouts
        let client = Client::builder().connect_timeout(connection_out).timeout(req_timeout).build()?;
        let mut envoy = Self { status: ECCStatus::Disconnected, id: id, connection: client };


        //Send a get request to establish the connection and retrieve the status
        let response: Response = envoy.connection.get(&envoy.id).send().await?;
        let message = envoy.parse_ecc_body(response)?;
        envoy.status = message.status;
        Ok(envoy)
    }

    pub async fn submit_get(&self) -> Result<ECCMessage, EnvoyError> {
        let response = self.connection.get(&self.id).send().await?;
        let message = self.parse_ecc_body(response)?;
        Ok(message)
    }

    pub async fn submit_post(&self) -> Result<ECCMessage, EnvoyError> {
        let response = self.connection.post(&self.id).send().await?;
        let message = self.parse_ecc_body(response)?;
        Ok(message)
    }

    fn parse_ecc_body(&self, response: Response) -> Result<ECCMessage, EnvoyError> {
        todo!()
    }

    fn compose_ecc_body(&self, message: ECCMessage) -> Result<Response, EnvoyError> {
        todo!()
    }
}