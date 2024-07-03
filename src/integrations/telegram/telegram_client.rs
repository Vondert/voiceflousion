use std::ops::Deref;
use std::sync::Arc;
use async_trait::async_trait;
use crate::integrations::session_map::SessionMap;
use crate::integrations::{Session, TelegramSession};
use crate::integrations::client::Client;
use crate::integrations::interaction_type::InteractionType;
use crate::integrations::locked_session::LockedSession;
use crate::integrations::telegram::telegram_update::TelegramUpdate;
use crate::integrations::update::Update;
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
    type ClientUpdate = TelegramUpdate;

    async fn launch_voiceflow_dialog(&self, locked_session: &LockedSession<Self::ClientSession>,  interaction_time: i64, state: Option<State>) -> Result<VoiceflowMessage, VoiceflowError>{
        let voiceflow_session = &**locked_session.session();
        let message = self.voiceflow_client.launch_dialog(voiceflow_session, state).await?;
        //session.set_last_interaction(interaction_time).await;
        Ok(message)
    }
    async fn send_message_to_voiceflow_dialog(&self, locked_session: &LockedSession<Self::ClientSession>,  interaction_time: i64, message: String, state: Option<State>) -> Result<VoiceflowMessage, VoiceflowError> {
        locked_session.set_last_interaction(interaction_time).await;
        let voiceflow_session = &**locked_session.session();
        self.voiceflow_client.send_message(voiceflow_session, state, message).await
    }
    async fn choose_button_in_voiceflow_dialog(&self,locked_session: &LockedSession<Self::ClientSession>,  interaction_time: i64, button_name: String, state: Option<State>) -> Result<VoiceflowMessage, VoiceflowError> {
        locked_session.set_last_interaction(interaction_time).await;
        let voiceflow_session = &**locked_session.session();
        self.voiceflow_client.choose_button(voiceflow_session, state, button_name).await
    }
    async fn interact_with_client(&self, update: Self::ClientUpdate, launch_state: Option<State>, update_state: Option<State>) -> Result<VoiceflowMessage, VoiceflowError>{
        let interaction_time =  update.interaction_time();
        if let Some(telegram_session) = self.get_session(update.chat_id_cloned()).await {
            let locked_session = LockedSession::try_from_session(&telegram_session)?;
            let is_valid = locked_session.is_valid(&self.session_duration).await;
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
            let telegram_session = self.add_session(update.chat_id_cloned()).await;
            let locked_session = LockedSession::try_from_session(&telegram_session)?;
            self.launch_voiceflow_dialog(&locked_session, interaction_time, launch_state).await
        }
    }

    fn session_duration(&self) -> Option<i64> {
        self.session_duration
    }
}