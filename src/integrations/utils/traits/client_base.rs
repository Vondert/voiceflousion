use std::sync::Arc;
use crate::integrations::utils::session_map::SessionMap;
use crate::integrations::utils::traits::{Sender, Session, Update};
use crate::voiceflow::VoiceflowClient;

pub trait ClientBase: Sync + Send {
    type ClientSession: Session;
    type ClientUpdate: Update;
    type ClientSender: Sender;
    fn client_id(&self) -> &String;
    fn sessions(&self) -> &Arc<SessionMap<Self::ClientSession>>;
    fn voiceflow_client(&self) -> &Arc<VoiceflowClient>;
    fn sender(&self) -> &Self::ClientSender;
}