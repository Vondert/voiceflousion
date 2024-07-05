use std::sync::Arc;
use async_trait::async_trait;
use crate::integrations::locked_session::LockedSession;
use crate::integrations::session_map::SessionMap;
use crate::integrations::TelegramSession;
use crate::integrations::utils::traits::{Session, SessionBase};
use crate::integrations::utils::traits::update::Update;
use crate::voiceflow::dialog_blocks::VoiceflowMessage;
use crate::voiceflow::request_structures::State;
use crate::voiceflow::{VoiceflowClient, VoiceflowError};

#[async_trait]
pub trait Client{
    type ClientSession: Session;
    type ClientUpdate: Update;
    fn sessions(&self) -> &SessionMap<Self::ClientSession>;
    fn session_duration(&self) -> Option<i64>;
    fn voiceflow_client(&self) -> &Arc<VoiceflowClient>;
    async fn launch_voiceflow_dialog(&self, locked_session: &LockedSession<Self::ClientSession>,  interaction_time: i64, state: Option<State>) -> Result<VoiceflowMessage, VoiceflowError>{
        let voiceflow_session = locked_session.voiceflow_session();
        let message = self.voiceflow_client().launch_dialog(voiceflow_session, state).await?;
        locked_session.set_last_interaction(interaction_time).await;
        Ok(message)
    }
    async fn send_message_to_voiceflow_dialog(&self, locked_session: &LockedSession<Self::ClientSession>,  interaction_time: i64, message: String, state: Option<State>) -> Result<VoiceflowMessage, VoiceflowError> {
        locked_session.set_last_interaction(interaction_time).await;
        let voiceflow_session = locked_session.voiceflow_session();
        self.voiceflow_client().send_message(voiceflow_session, state, message).await
    }

    async fn choose_button_in_voiceflow_dialog(&self,locked_session: &LockedSession<Self::ClientSession>,  interaction_time: i64, button_name: String, state: Option<State>) -> Result<VoiceflowMessage, VoiceflowError> {
        locked_session.set_last_interaction(interaction_time).await;
        let voiceflow_session = locked_session.voiceflow_session();
        self.voiceflow_client().choose_button(voiceflow_session, state, button_name).await
    }
    async fn interact_with_client(&self, update: Self::ClientUpdate, launch_state: Option<State>, update_state: Option<State>) -> Result<VoiceflowMessage, VoiceflowError>;
}