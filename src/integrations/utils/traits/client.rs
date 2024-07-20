use async_trait::async_trait;
use crate::integrations::utils::session_wrappers::LockedSession;
use crate::integrations::utils::subtypes::SentMessage;
use crate::integrations::utils::traits::{ClientBase, Responder, Sender};
use crate::voiceflow::{State, VoiceflousionError, VoiceflowMessage};

/// A trait that extends `ClientBase` with additional functionalities.
///
/// The `Client` trait adds methods for launching dialogs, sending messages,
/// and choosing buttons in a Voiceflow dialog. It provides asynchronous methods
/// for interacting with the Voiceflow API and managing session states.
#[async_trait]
pub trait Client: ClientBase {
    /// Launches a dialog between Client and VoiceflowClient, sends VoiceflowClient response to Client.
    ///
    /// # Parameters
    ///
    /// * `locked_session` - The locked session for the interaction.
    /// * `interaction_time` - The interaction time.
    /// * `state` - The optional state for launching the dialog.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `SenderResponder` or a `VoiceflousionError` if the request fails.
    async fn launch_voiceflow_dialog(&self, locked_session: &LockedSession,  interaction_time: i64, state: Option<State>) -> Result<Vec<<Self::ClientSender as Sender>::SenderResponder>, VoiceflousionError>{
        locked_session.set_last_interaction(Some(interaction_time));
        let voiceflow_session = locked_session.voiceflow_session();
        let mut voiceflow_message = self.voiceflow_client().launch_dialog(voiceflow_session, state).await?;
        if voiceflow_message.trim_end_block(){
            locked_session.set_last_interaction(None);
        }
        let response = self.sender().send_message(locked_session.get_chat_id(), voiceflow_message).await?;
        let bot_last_message = get_last_sent_message(&response);
        locked_session.set_previous_message(bot_last_message).await;
        Ok(response)
    }

    /// Sends a message from Client a VoiceflowClient and sends VoiceflowClient response to Client.
    ///
    /// # Parameters
    ///
    /// * `locked_session` - The locked session for the interaction.
    /// * `interaction_time` - The interaction time.
    /// * `message` - The text message to send.
    /// * `state` - The optional state for updating the dialog.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `SenderResponder` or a `VoiceflousionError` if the request fails.
    async fn send_message_to_voiceflow_dialog(&self, locked_session: &LockedSession, interaction_time: i64, message: &String, state: Option<State>) -> Result<Vec<<Self::ClientSender as Sender>::SenderResponder>, VoiceflousionError> {
        locked_session.set_last_interaction(Some(interaction_time));
        let voiceflow_session = locked_session.voiceflow_session();
        let mut voiceflow_message = self.voiceflow_client().send_message(voiceflow_session, state, message).await?;
        if voiceflow_message.trim_end_block(){
            locked_session.set_last_interaction(None);
        }
        let response = self.sender().send_message(locked_session.get_chat_id(), voiceflow_message).await?;
        let bot_last_message = get_last_sent_message(&response);
        locked_session.set_previous_message(bot_last_message).await;
        Ok(response)
    }

    /// Sends message from Client to choose a button in a VoiceflowClient and sends VoiceflowClient response to Client.
    ///
    /// # Parameters
    ///
    /// * `locked_session` - The locked session for the interaction.
    /// * `interaction_time` - The interaction time.
    /// * `message` - The text message associated with the button.
    /// * `button_data` - The data associated with the button.
    /// * `state` - The optional state for updating the dialog.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `SenderResponder` or a `VoiceflousionError` if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// let response = client.choose_button_in_voiceflow_dialog(&locked_session, interaction_time, &message, &button_data, state).await?;
    /// ```
    async fn choose_button_in_voiceflow_dialog(&self, locked_session: &LockedSession,  interaction_time: i64, message: &String, button_data: &String, state: Option<State>) -> Result<Vec<<Self::ClientSender as Sender>::SenderResponder>, VoiceflousionError> {
        locked_session.set_last_interaction(Some(interaction_time));
        let voiceflow_session = locked_session.voiceflow_session();
        let mut voiceflow_message = self.voiceflow_client().choose_button(voiceflow_session, state, message, button_data).await?;
        if voiceflow_message.trim_end_block(){
            locked_session.set_last_interaction(None);
        }
        let response = self.sender().send_message(locked_session.get_chat_id(), voiceflow_message).await?;
        let bot_last_message = get_last_sent_message(&response);
        locked_session.set_previous_message(bot_last_message).await;
        Ok(response)
    }
    /// Interacts with the client based on the provided update.
    ///
    /// # Parameters
    ///
    /// * `update` - The update from the client.
    /// * `launch_state` - The optional state for launching the dialog.
    /// * `update_state` - The optional state for updating the dialog.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `SenderResponder` or a `VoiceflousionError` if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// let response = client.interact_with_client(update, launch_state, update_state).await?;
    /// ```
    async fn interact_with_client(&self, update: Self::ClientUpdate, launch_state: Option<State>, update_state: Option<State>) -> Result<Vec<<Self::ClientSender as Sender>::SenderResponder>, VoiceflousionError>;
}

/// Retrieves the last sent message from the response.
///
/// # Parameters
///
/// * `response` - The response containing sent messages.
///
/// # Returns
///
/// An `Option` containing the last `SentMessage` if available.
pub fn get_last_sent_message<R: Responder>(response: &[R]) -> Option<SentMessage>{
    return if let Some(responder) = response.last(){
        Some(responder.create_sent_message())
    }
    else{
        None
    };
}