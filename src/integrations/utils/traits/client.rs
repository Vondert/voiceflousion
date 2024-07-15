use async_trait::async_trait;
use crate::integrations::utils::sent_message::SentMessage;
use crate::integrations::utils::LockedSession;
use crate::integrations::utils::traits::{ClientBase, Responder, Sender, Session};
use crate::voiceflow::{State, VoiceflousionError};

#[async_trait]
pub trait Client: ClientBase {
    async fn launch_voiceflow_dialog(&self, locked_session: &LockedSession<Self::ClientSession>,  interaction_time: i64, state: Option<State>) -> Result<Vec<<Self::ClientSender as Sender>::SenderResponder>, VoiceflousionError>{
        let voiceflow_session = locked_session.voiceflow_session();
        let voiceflow_message = self.voiceflow_client().launch_dialog(voiceflow_session, state).await?;
        let response = self.sender().send_message(locked_session.get_chat_id(), voiceflow_message).await?;
        let bot_last_message = get_last_sent_message(&response);
        locked_session.set_last_interaction(interaction_time).await;
        locked_session.set_previous_message(bot_last_message).await;
        Ok(response)
    }
    async fn send_message_to_voiceflow_dialog(&self, locked_session: &LockedSession<Self::ClientSession>, interaction_time: i64, message: &String, state: Option<State>) -> Result<Vec<<Self::ClientSender as Sender>::SenderResponder>, VoiceflousionError> {
        locked_session.set_last_interaction(interaction_time).await;
        let voiceflow_session = locked_session.voiceflow_session();
        let voiceflow_message = self.voiceflow_client().send_message(voiceflow_session, state, message).await?;
        let response = self.sender().send_message(locked_session.get_chat_id(), voiceflow_message).await?;
        let bot_last_message = get_last_sent_message(&response);
        locked_session.set_previous_message(bot_last_message).await;
        Ok(response)
    }
    async fn choose_button_in_voiceflow_dialog(&self, locked_session: &LockedSession<Self::ClientSession>,  interaction_time: i64, message: &String, button_data: &String, state: Option<State>) -> Result<Vec<<Self::ClientSender as Sender>::SenderResponder>, VoiceflousionError> {
        locked_session.set_last_interaction(interaction_time).await;
        let voiceflow_session = locked_session.voiceflow_session();
        let voiceflow_message = self.voiceflow_client().choose_button(voiceflow_session, state, message, button_data).await?;
        let response = self.sender().send_message(locked_session.get_chat_id(), voiceflow_message).await?;
        let bot_last_message = get_last_sent_message(&response);
        locked_session.set_previous_message(bot_last_message).await;
        Ok(response)
    }
    async fn interact_with_client(&self, update: Self::ClientUpdate, launch_state: Option<State>, update_state: Option<State>) -> Result<Vec<<Self::ClientSender as Sender>::SenderResponder>, VoiceflousionError>;
}

fn get_last_sent_message<R: Responder>(response: &[R]) -> Option<SentMessage>{
    return if let Some(responder) = response.last(){
        Some(responder.create_sent_message())
    }
    else{
        None
    };
}