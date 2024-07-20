use std::sync::Arc;
use crate::integrations::utils::sessions_manager::SessionsManager;
use crate::integrations::utils::traits::{Sender, Update};
use crate::voiceflow::VoiceflowClient;

pub trait ClientBase: Sync + Send {
    type ClientUpdate: Update;
    type ClientSender: Sender;
    fn client_id(&self) -> &String;
    fn sessions(&self) -> &Arc<SessionsManager>;
    fn voiceflow_client(&self) -> &Arc<VoiceflowClient>;
    fn sender(&self) -> &Self::ClientSender;
}