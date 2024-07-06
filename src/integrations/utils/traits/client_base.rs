use std::sync::Arc;
use crate::integrations::utils::session_map::SessionMap;
use crate::integrations::utils::traits::{Message, Sender, Session};
use crate::integrations::utils::traits::update::Update;
use crate::voiceflow::VoiceflowClient;

pub trait ClientBase {
    type ClientSession: Session;
    type ClientUpdate: Update;
    type ClientMessage: Message;
    type ClientSender: Sender<Self::ClientMessage>;
    fn sessions(&self) -> &SessionMap<Self::ClientSession>;
    fn session_duration(&self) -> &Option<i64>;
    fn voiceflow_client(&self) -> &Arc<VoiceflowClient>;
    fn sender(&self) -> &Self::ClientSender;
}