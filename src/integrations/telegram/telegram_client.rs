use std::sync::Arc;
use async_trait::async_trait;
use crate::integrations::utils::traits::{Session, ClientBase, Client};
use crate::integrations::telegram::telegram_update::TelegramUpdate;
use crate::integrations::telegram::TelegramSession;
use crate::integrations::utils::{InteractionType, LockedSession, SessionMap};
use crate::integrations::utils::traits::Update;
use crate::voiceflow::{VoiceflowClient, VoiceflowError, VoiceflowMessage};
use crate::voiceflow::request_structures::State;

pub struct TelegramClient{
    bot_id: String,
    bot_token: String,
    voiceflow_client: Arc<VoiceflowClient>,
    sessions: SessionMap<TelegramSession>,
    session_duration: Option<i64>
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
impl ClientBase for TelegramClient {
    type ClientSession = TelegramSession;
    type ClientUpdate = TelegramUpdate;

    fn sessions(&self) -> &SessionMap<Self::ClientSession> {
        &self.sessions
    }
    fn session_duration(&self) -> &Option<i64> {
        &self.session_duration
    }
    fn voiceflow_client(&self) -> &Arc<VoiceflowClient> {
        &self.voiceflow_client
    }
}
#[async_trait]
impl Client for TelegramClient{
    async fn interact_with_client(&self, update: Self::ClientUpdate, launch_state: Option<State>, update_state: Option<State>) -> Result<VoiceflowMessage, VoiceflowError>{
        let interaction_time =  update.interaction_time();
        if let Some(telegram_session) = self.sessions().get_session(update.chat_id_cloned()).await {
            let locked_session = LockedSession::try_from_session(&telegram_session)?;
            let is_valid = locked_session.is_valid(&self.session_duration()).await;
            if !is_valid {
                return self.launch_voiceflow_dialog(&locked_session, interaction_time, launch_state).await
            }
            match update.interaction_type(){
                InteractionType::Button(button_name) => {
                    self.choose_button_in_voiceflow_dialog(&locked_session, interaction_time, button_name, update_state).await
                },
                InteractionType::Undefined(message) | InteractionType::Text(message) => {
                    self.send_message_to_voiceflow_dialog(&locked_session,interaction_time, message, update_state).await
                }
            }
        }
        else{
            let telegram_session = self.sessions().add_session(update.chat_id_cloned()).await;
            let locked_session = LockedSession::try_from_session(&telegram_session)?;
            self.launch_voiceflow_dialog(&locked_session, interaction_time, launch_state).await
        }
    }
}