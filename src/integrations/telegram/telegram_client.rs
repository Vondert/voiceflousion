use std::ops::Deref;
use std::sync::Arc;
use async_trait::async_trait;
use crate::integrations::utils::traits::{ClientBase, Client, Update, Sender};
use crate::integrations::telegram::{TelegramResponder, TelegramSender, TelegramUpdate};
use crate::integrations::utils::{ClientBuilder, SessionsManager};
use crate::integrations::utils::session_wrappers::LockedSession;
use crate::integrations::utils::subtypes::InteractionType;
use crate::voiceflow::{State, VoiceflousionError, VoiceflowBlock, VoiceflowClient};
use crate::voiceflow::dialog_blocks::VoiceflowCarousel;

pub struct TelegramClient{
    client_id: String,
    voiceflow_client: Arc<VoiceflowClient>,
    sessions: Arc<SessionsManager>,
    sender: TelegramSender
}
impl TelegramClient{
    pub fn new(builder: ClientBuilder) -> Self{
        let client_id = builder.client_id().clone();
        let api_key = builder.api_key().clone();
        let voiceflow_client = builder.voiceflow_client().clone();
        let max_connections_per_moment = builder.max_connections_per_moment();
        let is_cleaning = builder.is_cleaning();
        let session_duration = builder.session_duration();
        let sessions_cleanup_interval = builder.sessions_cleanup_interval();
        let sessions= builder.sessions();

        Self{
            client_id,
            voiceflow_client,
            sessions: Arc::new(SessionsManager::new(sessions, session_duration, sessions_cleanup_interval, is_cleaning)),
            sender: TelegramSender::new(max_connections_per_moment, api_key)
        }
    }
    pub async fn switch_carousel_card(&self, locked_session: &LockedSession<'_>,  carousel: &VoiceflowCarousel,  message_id: &String, index: usize, interaction_time: i64) -> Result<TelegramResponder, VoiceflousionError> {
        locked_session.set_last_interaction(Some(interaction_time));
        self.sender.update_carousel(carousel, index, locked_session.get_chat_id(), message_id).await
    }
}
impl ClientBase for TelegramClient {
    type ClientUpdate = TelegramUpdate;
    type ClientSender = TelegramSender;

    fn client_id(&self) -> &String {
        &self.client_id
    }
    fn sessions(&self) -> &Arc<SessionsManager> {
        &self.sessions
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
    async fn interact_with_client(&self, update: Self::ClientUpdate, launch_state: Option<State>, update_state: Option<State>) -> Result<Vec<<Self::ClientSender as Sender>::SenderResponder>, VoiceflousionError>{
        let interaction_time =  update.interaction_time();
        if let Some(telegram_session) = self.sessions().get_session(update.chat_id()).await {
            let locked_session = LockedSession::try_from_session(&telegram_session)?;
            if let Some(message) = locked_session.previous_message().await.deref(){
                update.is_deprecated(message.date())?
            }
            match update.interaction_type(){
                InteractionType::Button(message, button_path) => {
                    if let Some(message) = locked_session.previous_message().await.deref() {
                        if let VoiceflowBlock::Carousel(carousel) = message.block() {
                            if let Some(index) = update.carousel_card_index() {
                                return Ok(vec![self.switch_carousel_card(&locked_session, carousel, message.id(), index, interaction_time).await?])
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