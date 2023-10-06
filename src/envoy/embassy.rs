use super::message::{EmbassyMessage, ECCMessage, MessageKind};
use super::error::EmbassyError;
use tokio::sync::mpsc::{Sender, Receiver, error::TryRecvError};

#[derive(Debug)]
pub struct Embassy {
    ecc_senders: Vec<Sender<EmbassyMessage>>,
    ecc_reciever: Receiver<ECCMessage>
}

impl Embassy {

    pub fn new(ecc_reciever: Receiver<ECCMessage>, ecc_senders: Vec<Sender<EmbassyMessage>>) -> Self {
        Embassy { ecc_senders: ecc_senders, ecc_reciever: ecc_reciever }
    }

    pub fn submit_message(self, message: EmbassyMessage) -> Result<(), EmbassyError> {
        if message.kind == MessageKind::ECC {
            for sender in self.ecc_senders.iter() {
                sender.blocking_send(message.clone())?;
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