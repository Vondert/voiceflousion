use async_trait::async_trait;
use reqwest::Response;
use crate::integrations::utils::{InteractionType, LockedSession};
use crate::integrations::utils::traits::{ClientBase, Sender, Session, Update};
use crate::voiceflow::{State, VoiceflousionError, VoiceflowMessage};

#[async_trait]
pub trait Client: ClientBase {
    async fn launch_voiceflow_dialog(&self, locked_session: &LockedSession<Self::ClientSession>,  interaction_time: i64, state: Option<State>) -> Result<Vec<Response>, VoiceflousionError>{
        let voiceflow_session = locked_session.voiceflow_session();
        let voiceflow_message = self.voiceflow_client().launch_dialog(voiceflow_session, state).await?;
        locked_session.set_last_interaction(interaction_time).await;
        self.sender().send_message(locked_session.get_chat_id(), voiceflow_message).await
    }
    async fn send_message_to_voiceflow_dialog(&self, locked_session: &LockedSession<Self::ClientSession>,  interaction_time: i64, message: String, state: Option<State>) -> Result<Vec<Response>, VoiceflousionError> {
        locked_session.set_last_interaction(interaction_time).await;
        let voiceflow_session = locked_session.voiceflow_session();
        let voiceflow_message = self.voiceflow_client().send_message(voiceflow_session, state, message).await?;
        self.sender().send_message(locked_session.get_chat_id(), voiceflow_message).await
    }
    async fn choose_button_in_voiceflow_dialog(&self,locked_session: &LockedSession<Self::ClientSession>,  interaction_time: i64, message: String, button_path: String, state: Option<State>) -> Result<Vec<Response>, VoiceflousionError> {
        locked_session.set_last_interaction(interaction_time).await;
        let voiceflow_session = locked_session.voiceflow_session();
        let voiceflow_message = self.voiceflow_client().choose_button(voiceflow_session, state, message, button_path).await?;
        self.sender().send_message(locked_session.get_chat_id(), voiceflow_message).await
    }
    async fn interact_with_client(&self, update: Self::ClientUpdate, launch_state: Option<State>, update_state: Option<State>) -> Result<Vec<Response>, VoiceflousionError>{
        let interaction_time =  update.interaction_time();
        if let Some(telegram_session) = self.sessions().get_session(update.chat_id_cloned()).await {
            let locked_session = LockedSession::try_from_session(&telegram_session)?;
            let is_valid = locked_session.is_valid(&self.session_duration()).await;
            if !is_valid {
               return self.launch_voiceflow_dialog(&locked_session, interaction_time, launch_state).await;
            }
            match update.interaction_type(){
                InteractionType::Button(message, button_path) => {
                    self.choose_button_in_voiceflow_dialog(&locked_session, interaction_time, message, button_path, update_state).await
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