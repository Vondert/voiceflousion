use std::ops::Deref;
use std::sync::Arc;
use async_trait::async_trait;
use reqwest::Response;
use crate::integrations::utils::traits::{ClientBase, Client, Update, Session};
use crate::integrations::telegram::{TelegramSender, TelegramSession, TelegramUpdate};
use crate::integrations::utils::{InteractionType, LockedSession, SessionMap};
use crate::voiceflow::{State, VoiceflousionError, VoiceflowBlock, VoiceflowClient};
use crate::voiceflow::dialog_blocks::VoiceflowCarousel;

pub struct TelegramClient{
    bot_id: String,
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
            voiceflow_client,
            sessions: SessionMap::from(telegram_session),
            session_duration,
            sender: TelegramSender::new(max_sessions_per_moment, bot_token)
        }
    }
    pub async fn switch_carousel_card(&self, locked_session: &LockedSession<'_, TelegramSession>,  carousel: &VoiceflowCarousel,  message_id: &String, index: usize, interaction_time: i64) -> Result<Response, VoiceflousionError> {
        locked_session.set_last_interaction(interaction_time).await;
        self.sender.update_carousel(carousel, index, locked_session.get_chat_id(), message_id).await
    }
}
impl ClientBase for TelegramClient {
    type ClientSession = TelegramSession;
    type ClientUpdate = TelegramUpdate;
    //type ClientMessage = TelegramMessage;
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
        &self.sender
    }
}
#[async_trait]
impl Client for TelegramClient{
    async fn interact_with_client(&self, update: Self::ClientUpdate, launch_state: Option<State>, update_state: Option<State>) -> Result<Vec<Response>, VoiceflousionError>{
        let interaction_time =  update.interaction_time();
        if let Some(telegram_session) = self.sessions().get_session(update.chat_id()).await {
            let locked_session = LockedSession::try_from_session(&telegram_session)?;
            let is_valid = locked_session.is_valid(&self.session_duration()).await;
            if !is_valid {
                return self.launch_voiceflow_dialog(&locked_session, interaction_time, launch_state).await;
            }
            match update.interaction_type(){
                InteractionType::Button(message, button_path) => {
                    if let Some(block) = locked_session.previous_message().await.deref() {
                        if let VoiceflowBlock::Carousel(carousel) = block {
                            if let Some(index) = update.carousel_card_index() {
                                return Ok(vec![self.switch_carousel_card(&locked_session, carousel, &"".to_string(), index, interaction_time).await?])
                            }
                        }
                    }
                    self.choose_button_in_voiceflow_dialog(&locked_session, interaction_time, message, button_path, update_state).await
                },
                InteractionType::Text(message) | InteractionType::Undefined(message) => {
                    self.send_message_to_voiceflow_dialog(&locked_session, interaction_time, message, update_state).await
                }
            }
        }
        else{
            let telegram_session = self.sessions().add_session(update.chat_id().clone()).await;
            let locked_session = LockedSession::try_from_session(&telegram_session)?;
            self.launch_voiceflow_dialog(&locked_session, interaction_time, launch_state).await
        }
    }
}