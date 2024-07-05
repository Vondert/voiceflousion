use async_trait::async_trait;
use crate::integrations::LockedSession;
use crate::integrations::utils::traits::{ClientBase, Session, SessionBase};
use crate::voiceflow::dialog_blocks::VoiceflowMessage;
use crate::voiceflow::{State, VoiceflowError};

#[async_trait]
pub trait Client: ClientBase {
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