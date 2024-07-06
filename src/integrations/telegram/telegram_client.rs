use std::sync::Arc;
use async_trait::async_trait;
use crate::integrations::utils::traits::{Session, ClientBase, Client, Update, Sender};
use crate::integrations::telegram::{TelegramMessage, TelegramSender, TelegramSession, TelegramUpdate};
use crate::integrations::utils::{InteractionType, LockedSession, SessionMap};
use crate::voiceflow::{VoiceflowClient, VoiceflousionError};
use crate::voiceflow::request_structures::State;

pub struct TelegramClient{
    bot_id: String,
    bot_token: String,
    voiceflow_client: Arc<VoiceflowClient>,
    sessions: SessionMap<TelegramSession>,
    session_duration: Option<i64>,
    sender: TelegramSender
}
impl TelegramClient{
    pub fn new(bot_token: String, voiceflow_client: Arc<VoiceflowClient>, telegram_session: Option<Vec<TelegramSession>>, session_duration: Option<i64>, max_sessions_per_moment: usize) -> Self{
        let bot_id = bot_token.split(':').next().unwrap().to_string();
        Self{
            bot_id,
            bot_token,
            voiceflow_client,
            sessions: SessionMap::from(telegram_session),
            session_duration,
            sender: TelegramSender::new(max_sessions_per_moment)
        }
    }
}
impl ClientBase for TelegramClient {
    type ClientSession = TelegramSession;
    type ClientUpdate = TelegramUpdate;
    type ClientMessage = TelegramMessage;
    type ClientSender = TelegramSender;
    fn sessions(&self) -> &SessionMap<Self::ClientSession> {
        &self.sessions
    }
    fn session_duration(&self) -> &Option<i64> {
        &self.session_duration
    }
    fn voiceflow_client(&self) -> &Arc<VoiceflowClient> {
        &self.voiceflow_client
    }

    fn sender(&self) -> &Self::ClientSender {
        todo!()
    }
}
#[async_trait]
impl Client for TelegramClient{}