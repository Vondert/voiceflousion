use std::ops::Deref;
use std::sync::Arc;
use async_trait::async_trait;
use crate::integrations::session_map::SessionMap;
use crate::integrations::{Session, TelegramSession};
use crate::integrations::client::Client;
use crate::voiceflow::{VoiceflowClient, VoiceflowError};
use crate::voiceflow::dialog_blocks::VoiceflowMessage;
use crate::voiceflow::request_structures::State;

pub struct TelegramClient{
    bot_id: String,
    bot_token: String,
    voiceflow_client: Arc<VoiceflowClient>,
    sessions: SessionMap<TelegramSession>,
    session_duration: Option<i64>
}
impl Deref for TelegramClient{
    type Target = SessionMap<TelegramSession>;

    fn deref(&self) -> &Self::Target {
        &self.sessions
    }
}
impl TelegramClient{
    pub fn new(bot_token: String, voiceflow_client: Arc<VoiceflowClient>, telegram_session: Option<Vec<TelegramSession>>, session_duration: Option<i64>) -> Self{
        let bot_id = bot_token.split(':').next().unwrap().to_string();
        Self{
            bot_id,
            bot_token,
            voiceflow_client,
            sessions: SessionMap::from(telegram_session),
            session_duration
        }
    }
}
#[async_trait]
impl Client for TelegramClient {
    type ClientSession = TelegramSession;

    async fn launch_voiceflow_dialog(&self, session: &Self::ClientSession,  interaction_time: i64, state: Option<State>) -> Result<VoiceflowMessage, VoiceflowError>{
        let message = self.voiceflow_client.launch_dialog(session, state).await?;
        session.set_last_interaction_locked(interaction_time)?;
        Ok(message)
    }
    async fn send_message_to_voiceflow_dialog(&self, session: &Self::ClientSession,  interaction_time: i64, message: String, state: Option<State>) -> Result<VoiceflowMessage, VoiceflowError> {
        session.set_last_interaction_locked(interaction_time)?;
        self.voiceflow_client.send_message(session, state, message).await
    }

    async fn interact_with_client(&self, chat_id: String, interaction_time: i64, message: String, launch_state: Option<State>, update_state: Option<State>) -> Result<VoiceflowMessage, VoiceflowError>{
        if let Some(telegram_session) = self.get_session(chat_id.clone()).await {
            let _guard = telegram_session.try_lock()?;
            let is_valid = self.is_valid_session(&*telegram_session)?;
            if !is_valid {
                return self.launch_voiceflow_dialog(&*telegram_session, interaction_time, launch_state).await
            }
            self.send_message_to_voiceflow_dialog(&*telegram_session, interaction_time, message, update_state).await
        }
        else{
            let telegram_session = self.add_session(chat_id).await;
            let _guard = telegram_session.try_lock()?;
            self.launch_voiceflow_dialog(&*telegram_session, interaction_time, launch_state).await
        }
    }

    fn session_duration(&self) -> Option<i64> {
        self.session_duration
    }
}