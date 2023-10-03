use super::ecc_envoy::ECCEnvoy;
use super::message::EmbassyMessage;

use std::sync::mpsc::Sender;

#[derive(Debug)]
pub struct Embassy {
    ecc_envoys: Vec<ECCEnvoy>,
    transmitter: Sender<EmbassyMessage>
}

impl Embassy {

}