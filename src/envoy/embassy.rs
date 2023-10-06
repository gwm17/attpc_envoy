use super::message::{EmbassyMessage, MessageKind};
use super::error::EmbassyError;
use std::collections::HashMap;
use tokio::sync::mpsc::{Sender, Receiver, error::TryRecvError};

#[derive(Debug)]
pub struct Embassy {
    ecc_senders: HashMap<i32, Sender<EmbassyMessage>>,
    ecc_reciever: Receiver<EmbassyMessage>
}

impl Embassy {

    pub fn new(ecc_reciever: Receiver<EmbassyMessage>, ecc_senders: HashMap<i32, Sender<EmbassyMessage>>) -> Self {
        Embassy { ecc_senders, ecc_reciever }
    }

    pub fn submit_message(&mut self, message: EmbassyMessage) -> Result<(), EmbassyError> {
        if message.kind == MessageKind::ECC {
            if let Some(&mut sender) = self.ecc_senders.get_mut(&message.id) {
                sender.blocking_send(message)?;
            }
        }
        Ok(())
    }

    pub fn poll_messages(&mut self) -> Result<Vec<EmbassyMessage>, EmbassyError> {
        let mut messages: Vec<EmbassyMessage> = vec![];
        loop {
            match self.ecc_reciever.try_recv() {
                Ok(message) => messages.push(message.into()),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => return Err(EmbassyError::MessageRecieveError)
            };
        }
        Ok(messages)
    }


}