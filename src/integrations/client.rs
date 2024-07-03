use std::ops::Deref;
use async_trait::async_trait;
use chrono::Utc;
use crate::integrations::{Session, TelegramSession};
use crate::integrations::locked_session::LockedSession;
use crate::integrations::update::Update;
use crate::voiceflow::dialog_blocks::VoiceflowMessage;
use crate::voiceflow::request_structures::State;
use crate::voiceflow::VoiceflowError;

#[async_trait]
pub trait Client: Deref{
    type ClientSession: Session;
    type ClientUpdate: Update;
    async fn launch_voiceflow_dialog(&self, locked_session: &LockedSession<Self::ClientSession>,  interaction_time: i64, state: Option<State>) -> Result<VoiceflowMessage, VoiceflowError>;
    async fn send_message_to_voiceflow_dialog(&self, locked_session: &LockedSession<Self::ClientSession>,  interaction_time: i64, message: String, state: Option<State>) -> Result<VoiceflowMessage, VoiceflowError>;
    async fn choose_button_in_voiceflow_dialog(&self, locked_session: &LockedSession<Self::ClientSession>,  interaction_time: i64, button_name: String, state: Option<State>) -> Result<VoiceflowMessage, VoiceflowError>;
    async fn interact_with_client(&self, update: Self::ClientUpdate, launch_state: Option<State>, update_state: Option<State>) -> Result<VoiceflowMessage, VoiceflowError>;
    fn session_duration(&self) -> Option<i64>;

}