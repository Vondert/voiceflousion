use std::ops::Deref;
use std::vec;
use async_trait::async_trait;
use crate::core::ClientBuilder;
use crate::core::session_wrappers::LockedSession;
use crate::core::subtypes::{InteractionType, SentMessage};
use crate::core::traits::{ClientBase, Responder, Sender, Update};
use crate::core::voiceflow::{State, VoiceflousionError, VoiceflowBlock, VoiceflowMessage};

/// A trait that extends `ClientBase` with additional functionalities.
///
/// The `Client` trait adds methods for launching dialogs, sending messages,
/// and choosing buttons in a Voiceflow dialog. It provides asynchronous methods
/// for interacting with the Voiceflow API and managing session states.
#[async_trait]
pub trait Client: ClientBase {
    /// Launches a dialog between Client and VoiceflowClient, sends VoiceflowClient response to Client.
    ///
    /// **This method has base implementation for sending messages. Modify it only if you
    /// know what you are doing or have devised a better approach.**
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
    ///
    /// # Example
    ///
    /// ```
    /// let response = client.launch_voiceflow_dialog(&locked_session, interaction_time, state).await?;
    /// ```
    async fn launch_voiceflow_dialog(&self, locked_session: &LockedSession,  interaction_time: i64) -> Result<Vec<<Self::ClientSender as Sender>::SenderResponder>, VoiceflousionError>{
        // Set the last interaction time for the session
        locked_session.set_last_interaction(Some(interaction_time));

        // Get the Voiceflow session associated with the locked session
        let voiceflow_session = locked_session.voiceflow_session();

        // Launch a new dialog with the Voiceflow client
        let mut voiceflow_message = self.voiceflow_client().launch_dialog(voiceflow_session, Some(State::new(vec![]))).await;

        // If the Voiceflow message indicates the end of the block, clear the last interaction time to make session invalid
        if voiceflow_message.trim_end_block() {
            locked_session.set_last_interaction(None);
        }

        // Send the Voiceflow message to the client and get the response
        let response = self.sender().send_message(locked_session.get_chat_id(), voiceflow_message).await?;

        // Retrieve the last message sent by the bot from the response
        let bot_last_message = get_last_sent_message(&response);

        // Update the session with the previous message
        locked_session.set_previous_message(bot_last_message).await;

        // Return the response
        Ok(response)
    }

    /// Sends a message from Client a VoiceflowClient and sends VoiceflowClient response to Client.
    ///
    /// **This method has base implementation for sending messages. Modify it only if you
    /// know what you are doing or have devised a better approach.**
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
    ///
    /// # Example
    ///
    /// ```
    /// let response = client.send_message_to_voiceflow_dialog(&locked_session, interaction_time, message, state).await?;
    /// ```
    async fn send_message_to_voiceflow_dialog(&self, locked_session: &LockedSession, interaction_time: i64, message: &String, state: Option<State>) -> Result<Vec<<Self::ClientSender as Sender>::SenderResponder>, VoiceflousionError> {
        // Set the last interaction time for the session
        locked_session.set_last_interaction(Some(interaction_time));

        // Get the Voiceflow session associated with the locked session
        let voiceflow_session = locked_session.voiceflow_session();

        // Send the message to the Voiceflow client
        let mut voiceflow_message = self.voiceflow_client().send_message(voiceflow_session, state, message).await;

        // If the Voiceflow message indicates the end of the block, clear the last interaction time to make session invalid
        if voiceflow_message.trim_end_block() {
            locked_session.set_last_interaction(None);
        }

        // Send the Voiceflow message to the client and get the response
        let response = self.sender().send_message(locked_session.get_chat_id(), voiceflow_message).await?;

        // Retrieve the last message sent by the bot from the response
        let bot_last_message = get_last_sent_message(&response);

        // Update the session with the previous message
        locked_session.set_previous_message(bot_last_message).await;

        // Return the response
        Ok(response)
    }

    /// Sends message from Client to choose a button in a VoiceflowClient and sends VoiceflowClient response to Client.
    ///
    /// **This method has base implementation for sending messages. Modify it only if you
    /// know what you are doing or have devised a better approach.**
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
        // Set the last interaction time for the session
        locked_session.set_last_interaction(Some(interaction_time));

        // Get the Voiceflow session associated with the locked session
        let voiceflow_session = locked_session.voiceflow_session();

        // Send the button data to the Voiceflow client
        let mut voiceflow_message = self.voiceflow_client().choose_button(voiceflow_session, state, message, button_data).await;

        // If the Voiceflow message indicates the end of the block, clear the last interaction time to make session invalid
        if voiceflow_message.trim_end_block() {
            locked_session.set_last_interaction(None);
        }

        // Send the Voiceflow message to the client and get the response
        let response = self.sender().send_message(locked_session.get_chat_id(), voiceflow_message).await?;

        // Retrieve the last message sent by the bot from the response
        let bot_last_message = get_last_sent_message(&response);

        // Update the session with the previous message
        locked_session.set_previous_message(bot_last_message).await;

        // Return the response
        Ok(response)
    }
    /// Interacts with the client based on the provided update.
    ///
    /// **This method has base implementation for sending messages. Modify it only if you
    /// know what you are doing or have devised a better approach.**
    ///
    /// # Parameters
    ///
    /// * `update` - The update from the Telegram client.
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
    async fn interact_with_client(&self, update: Self::ClientUpdate, update_state: Option<State>) -> Result<Vec<<Self::ClientSender as Sender>::SenderResponder>, VoiceflousionError> {
        // Get the interaction time from the update
        let interaction_time = update.interaction_time();

        // Check if a session exists for the given chat_id
        if let Some(telegram_session) = self.sessions().get_session(update.chat_id()).await {
            // Lock the session for safe access
            let locked_session = LockedSession::try_from_session(&telegram_session)?;

            // Check if the update is deprecated
            if let Some(message) = locked_session.previous_message().await.deref() {
                update.is_deprecated(message.date())?
            }

            // Handle the interaction based on its type
            match update.interaction_type() {
                // If it is a button press
                InteractionType::Button(message, button_path) => {
                    // Handle the button interaction
                    self.handle_button_interaction(&locked_session, interaction_time, message, button_path, update_state, &update).await
                },
                // If it is a text message or an undefined interaction
                InteractionType::Text(message) | InteractionType::Undefined(message) => {
                    // Handle the text message
                    self.send_message_to_voiceflow_dialog(&locked_session, interaction_time, message, update_state).await
                }
            }
        } else {
            // If no session exists, create a new session and launch the dialog
            let telegram_session = self.sessions().add_session(update.chat_id().clone()).await;
            let locked_session = LockedSession::try_from_session(&telegram_session)?;
            self.launch_voiceflow_dialog(&locked_session, interaction_time).await
        }
    }

    /// Handles button pressed on client.
    ///
    /// # Parameters
    ///
    /// * `locked_session` - The locked session for the interaction.
    /// * `interaction_time` - The interaction time.
    /// * `message` - The text message associated with the button.
    /// * `button_path` - The data associated with the button.
    /// * `update_state` - The optional state for updating the dialog.
    /// * `update` - The update from the Telegram client.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `SenderResponder` or a `VoiceflousionError` if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// struct MyClient {
    ///     client_id: String,
    ///     sessions: Arc<SessionsManager>,
    ///     voiceflow_client: Arc<VoiceflowClient>,
    ///     sender: MySender,
    /// }
    ///
    /// #[async_trait]
    /// impl Client for MyClient {
    ///     async fn handle_button_interaction(&self, locked_session: &LockedSession<'_>, interaction_time: i64, message: &String, button_path: &String, update_state: Option<State>, update: &Self::ClientUpdate) -> Result<Vec<<Self::ClientSender as Sender>::SenderResponder>, VoiceflousionError> {
    ///         // Implement the handling logic here.
    ///     }
    /// }
    /// ```
    async fn handle_button_interaction(&self, locked_session: &LockedSession<'_>, interaction_time: i64, message: &String, button_path: &String, update_state: Option<State>, update: &Self::ClientUpdate, ) -> Result<Vec<<Self::ClientSender as Sender>::SenderResponder>, VoiceflousionError>;
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