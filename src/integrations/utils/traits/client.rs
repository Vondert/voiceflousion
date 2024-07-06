use async_trait::async_trait;
use crate::integrations::utils::{InteractionType, LockedSession};
use crate::integrations::utils::traits::{ClientBase, Message, Sender, Session, SessionBase, Update};
use crate::voiceflow::{State, VoiceflousionError};

#[async_trait]
pub trait Client: ClientBase {
    async fn launch_voiceflow_dialog(&self, locked_session: &LockedSession<Self::ClientSession>,  interaction_time: i64, state: Option<State>) -> Result<Self::ClientMessage, VoiceflousionError>{
        let voiceflow_session = locked_session.voiceflow_session();
        let voiceflow_message = self.voiceflow_client().launch_dialog(voiceflow_session, state).await?;
        let client_message = Self::ClientMessage::from_voiceflow_message(voiceflow_message);
        locked_session.set_last_interaction(interaction_time).await;
        Ok(client_message)
    }
    async fn send_message_to_voiceflow_dialog(&self, locked_session: &LockedSession<Self::ClientSession>,  interaction_time: i64, message: String, state: Option<State>) -> Result<Self::ClientMessage, VoiceflousionError> {
        locked_session.set_last_interaction(interaction_time).await;
        let voiceflow_session = locked_session.voiceflow_session();
        let voiceflow_message = self.voiceflow_client().send_message(voiceflow_session, state, message).await?;
        let client_message = Self::ClientMessage::from_voiceflow_message(voiceflow_message);
        Ok(client_message)
    }
    async fn choose_button_in_voiceflow_dialog(&self,locked_session: &LockedSession<Self::ClientSession>,  interaction_time: i64, button_name: String, state: Option<State>) -> Result<Self::ClientMessage, VoiceflousionError> {
        locked_session.set_last_interaction(interaction_time).await;
        let voiceflow_session = locked_session.voiceflow_session();
        let voiceflow_message = self.voiceflow_client().choose_button(voiceflow_session, state, button_name).await?;
        let client_message = Self::ClientMessage::from_voiceflow_message(voiceflow_message);
        Ok(client_message)
    }
    async fn interact_with_client(&self, update: Self::ClientUpdate, launch_state: Option<State>, update_state: Option<State>) -> Result<(), VoiceflousionError>{
        let interaction_time =  update.interaction_time();
        let message = if let Some(telegram_session) = self.sessions().get_session(update.chat_id_cloned()).await {
            let locked_session = LockedSession::try_from_session(&telegram_session)?;
            let is_valid = locked_session.is_valid(&self.session_duration()).await;
            if !is_valid {
                self.launch_voiceflow_dialog(&locked_session, interaction_time, launch_state).await?
            }
            else{
                match update.interaction_type(){
                    InteractionType::Button(button_name) => {
                        self.choose_button_in_voiceflow_dialog(&locked_session, interaction_time, button_name, update_state).await?
                    },
                    InteractionType::Undefined(message) | InteractionType::Text(message) => {
                        self.send_message_to_voiceflow_dialog(&locked_session,interaction_time, message, update_state).await?
                    }
                }
            }
        }
        else{
            let telegram_session = self.sessions().add_session(update.chat_id_cloned()).await;
            let locked_session = LockedSession::try_from_session(&telegram_session)?;
            self.launch_voiceflow_dialog(&locked_session, interaction_time, launch_state).await?
        };
        self.sender().send(message).await
    }
}