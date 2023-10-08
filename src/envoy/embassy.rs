use super::message::{EmbassyMessage, MessageKind};
use super::ecc_envoy::startup_ecc_envoys;
use super::error::EmbassyError;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::sync::broadcast;

#[derive(Debug)]
pub struct Embassy {
    ecc_senders: HashMap<i32, mpsc::Sender<EmbassyMessage>>,
    ecc_reciever: mpsc::Receiver<EmbassyMessage>,
    cancel: broadcast::Sender<EmbassyMessage>
}

impl Embassy {

    pub fn new(ecc_reciever: mpsc::Receiver<EmbassyMessage>, ecc_senders: HashMap<i32, mpsc::Sender<EmbassyMessage>>, cancel: broadcast::Sender<EmbassyMessage>) -> Self {
        Embassy { ecc_senders, ecc_reciever, cancel }
    }

    pub fn shutdown(&mut self) {
        let cancel_message = EmbassyMessage::compose_cancel();
        self.cancel.send(cancel_message).expect("Some how all the envoys are already dead?");
    }

    pub fn submit_message(&mut self, message: EmbassyMessage) -> Result<(), EmbassyError> {
        if message.kind == MessageKind::ECC {
            if let Some(sender) = self.ecc_senders.get_mut(&message.id) {
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
                Err(mpsc::error::TryRecvError::Empty) => break,
                Err(mpsc::error::TryRecvError::Disconnected) => return Err(EmbassyError::MessageRecieveError)
            };
        }
        Ok(messages)
    }


}

pub fn connect_embassy(runtime: &mut tokio::runtime::Runtime, experiment: &str) -> (Embassy, Vec<tokio::task::JoinHandle<()>>) {
    let (ecc_tx, embassy_rx) = mpsc::channel::<EmbassyMessage>(10);
    let (cancel_tx, _) = broadcast::channel::<EmbassyMessage>(10);
    let (ecc_handles, ecc_switchboard) = startup_ecc_envoys(runtime, experiment, &ecc_tx, &cancel_tx);
    let embassy = Embassy::new(embassy_rx, ecc_switchboard, cancel_tx);
    return (embassy, ecc_handles);
}