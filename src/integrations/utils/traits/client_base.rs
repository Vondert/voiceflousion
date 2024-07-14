use std::sync::Arc;
use crate::integrations::utils::session_map::SessionMap;
use crate::integrations::utils::traits::{Sender, Session, Responder, Update};
use crate::voiceflow::VoiceflowClient;

pub trait ClientBase {
    type ClientSession: Session;
    type ClientUpdate: Update;
    type ClientSender: Sender;
    fn sessions(&self) -> &SessionMap<Self::ClientSession, <Self::ClientSender as Sender>::SenderResponder>;
    fn voiceflow_client(&self) -> &Arc<VoiceflowClient>;
    fn sender(&self) -> &Self::ClientSender;
}