use std::sync::Arc;
use crate::integrations::session_map::SessionMap;
use crate::integrations::utils::traits::Session;
use crate::integrations::utils::traits::update::Update;
use crate::voiceflow::VoiceflowClient;

pub trait ClientBase {
    type ClientSession: Session;
    type ClientUpdate: Update;
    fn sessions(&self) -> &SessionMap<Self::ClientSession>;
    fn session_duration(&self) -> &Option<i64>;
    fn voiceflow_client(&self) -> &Arc<VoiceflowClient>;
}