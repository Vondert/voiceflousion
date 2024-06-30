use std::ops::Deref;
use std::sync::Arc;
use crate::integrations::session_map::SessionMap;
use crate::integrations::{Session, TelegramSession};
use crate::voiceflow::{VoiceflowClient, VoiceflowError};
use crate::voiceflow::dialog_blocks::VoiceflowMessage;
use crate::voiceflow::request_structures::State;

pub(crate) struct TelegramClient{
    bot_id: String,
    bot_token: String,
    voiceflow_client: Arc<VoiceflowClient>,
    sessions: SessionMap<TelegramSession>
}
impl Deref for TelegramClient{
    type Target = SessionMap<TelegramSession>;

    fn deref(&self) -> &Self::Target {
        &self.sessions
    }
}
impl TelegramClient {
    pub fn new(bot_token: String, voiceflow_client: Arc<VoiceflowClient>, telegram_session: Option<Vec<TelegramSession>>) -> Self{
        let bot_id = bot_token.split(':').next().unwrap().to_string();
        Self{
            bot_id,
            bot_token,
            voiceflow_client,
            sessions: SessionMap::from(telegram_session)
        }
    }
    pub async fn launch_voiceflow_dialog(&self, chat_id: String, state: Option<State>) -> Result<VoiceflowMessage, VoiceflowError>{
        let telegram_session = self.sessions.get_or_add_session_async(chat_id).await;
        let _guard = telegram_session.try_lock()?;
        self.voiceflow_client.launch_dialog(&*telegram_session, state).await
    }
}