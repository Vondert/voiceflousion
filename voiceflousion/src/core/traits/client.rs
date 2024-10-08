use std::ops::Deref;
use async_trait::async_trait;
use crate::core::base_structs::ClientBase;
use crate::core::session_wrappers::LockedSession;
use crate::core::subtypes::{InteractionType, SentMessage};
use crate::core::traits::{Responder, Sender, Update};
use crate::core::voiceflow::{State, VoiceflowBlock};
use crate::errors::{VoiceflousionError, VoiceflousionResult};

/// The `Client` trait adds methods for launching dialogs, sending messages,
/// and choosing buttons in a Voiceflow dialog. It provides asynchronous methods
/// for interacting with the Voiceflow API and managing session states.
#[async_trait]
pub trait Client: Sync + Send {

    /// The associated update type that must implement the `Update` trait and be valid for the `'async_trait` lifetime.
    type ClientUpdate<'async_trait>: Update + 'async_trait;

    /// The associated sender type that must implement the `Sender` trait and be valid for the `'async_trait` lifetime.
    type ClientSender<'async_trait>: Sender + 'async_trait
    where Self: 'async_trait;

    /// Returns a reference to the `ClientBase`.
    ///
    /// # Returns
    ///
    /// A reference to the `ClientBase` instance.
    fn client_base(&self) -> &ClientBase<Self::ClientSender<'_>>;

    /// Launches a dialog between Client and VoiceflowClient, sends VoiceflowClient response to Client.
    ///
    /// **This method has a base implementation for sending messages. Modify it only if you
    /// know what you are doing or have devised a better approach.**
    ///
    /// # Parameters
    ///
    /// * `locked_session` - The locked session for the interaction.
    /// * `interaction_time` - The interaction time.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing a vector of `SenderResponder` or a `VoiceflousionError` if the request fails.
    async fn launch_voiceflow_dialog(&self, locked_session: &LockedSession,  interaction_time: i64) -> VoiceflousionResult<Vec<<Self::ClientSender<'_> as Sender>::SenderResponder>>{
        // Set the last interaction time for the session
        locked_session.set_last_interaction(Some(interaction_time));

        // Get the Voiceflow session associated with the locked session
        let voiceflow_session = locked_session.voiceflow_session();

        // Get launch state for Voiceflow bot
        let state = self.client_base().launch_state().clone();

        // Launch a new dialog with the Voiceflow client
        let mut voiceflow_message = self.client_base().voiceflow_client().launch_dialog(voiceflow_session, state).await;

        // If the Voiceflow message indicates the end of the block, clear the last interaction time to make session invalid
        if voiceflow_message.trim_end_block() {
            locked_session.set_last_interaction(None);
        }

        let client_id = self.client_base().client_id();

        // Send the Voiceflow message to the client and get the response
        let response = self.client_base().sender().send_message(client_id, locked_session.get_chat_id(), voiceflow_message).await?;

        // Retrieve the last message sent by the bot from the response
        let bot_last_message = get_last_sent_message(&response);

        // Update the session with the previous message
        locked_session.set_previous_message(bot_last_message).await;

        // Return the response
        Ok(response)
    }

    /// Sends a message from Client to VoiceflowClient and sends the VoiceflowClient response to Client.
    ///
    /// This method handles sending a text message to the Voiceflow client, processes the response,
    /// and updates the session state accordingly.
    ///
    /// **This method has a base implementation for sending messages. Modify it only if you
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
    /// A `VoiceflousionResult` containing a vector of `SenderResponder` or a `VoiceflousionError` if the request fails.
    async fn send_message_to_voiceflow_dialog(&self, locked_session: &LockedSession, interaction_time: i64, message: &String, state: Option<State>) -> VoiceflousionResult<Vec<<Self::ClientSender<'_> as Sender>::SenderResponder>> {
        // Set the last interaction time for the session
        locked_session.set_last_interaction(Some(interaction_time));

        // Get the Voiceflow session associated with the locked session
        let voiceflow_session = locked_session.voiceflow_session();

        // Send the message to the Voiceflow client
        let mut voiceflow_message = self.client_base().voiceflow_client().send_message(voiceflow_session, state, message).await;

        // If the Voiceflow message indicates the end of the block, clear the last interaction time to make session invalid
        if voiceflow_message.trim_end_block() {
            locked_session.set_last_interaction(None);
        }

        let client_id = self.client_base().client_id();

        // Send the Voiceflow message to the client and get the response
        let response = self.client_base().sender().send_message(client_id, locked_session.get_chat_id(), voiceflow_message).await?;

        // Retrieve the last message sent by the bot from the response
        let bot_last_message = get_last_sent_message(&response);

        // Update the session with the previous message
        locked_session.set_previous_message(bot_last_message).await;

        // Return the response
        Ok(response)
    }

    /// Sends a message from Client to choose a button in a VoiceflowClient and sends the VoiceflowClient response to Client.
    ///
    /// This method handles sending button data to the Voiceflow client, processes the response,
    /// and updates the session state accordingly.
    ///
    /// **This method has a base implementation for sending messages. Modify it only if you
    /// know what you are doing or have devised a better approach.**
    ///
    /// # Parameters
    ///
    /// * `locked_session` - The locked session for the interaction.
    /// * `interaction_time` - The interaction time.
    /// * `button_index` - The index of the button in the previous message.
    /// * `state` - The optional state for updating the dialog.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing a vector of `SenderResponder` or a `VoiceflousionError` if the request fails.
    async fn choose_button_in_voiceflow_dialog(&self, locked_session: &LockedSession,  interaction_time: i64, state: Option<State>, button_index: usize) -> VoiceflousionResult<Vec<<Self::ClientSender<'_> as Sender>::SenderResponder>> {
        // Set the last interaction time for the session
        locked_session.set_last_interaction(Some(interaction_time));

        // Get the Voiceflow session associated with the locked session
        let voiceflow_session = locked_session.voiceflow_session();

        let voiceflow_message = {
            let binding = locked_session.previous_message().await;
            let previous_message = binding.deref().as_ref()
                .ok_or_else(|| VoiceflousionError::ClientRequestError("Client".to_string(),"Button cannot be handled in the start of the conversation".to_string()))?;
            let voiceflow_button = previous_message.get_button(button_index.clone())?;

            let payload = voiceflow_button.payload().clone();

            // Send the button data to the Voiceflow client
            let mut voiceflow_message = self.client_base().voiceflow_client().choose_button(voiceflow_session, state, payload).await;

            if let Some(url_text) = voiceflow_button.get_url_text(){
                voiceflow_message.shift_block(VoiceflowBlock::Text(url_text));
            }

            // If the Voiceflow message indicates the end of the block, clear the last interaction time to make session invalid
            if voiceflow_message.trim_end_block() {
                locked_session.set_last_interaction(None);
            }
            voiceflow_message
        };

        let client_id = self.client_base().client_id();

        // Send the Voiceflow message to the client and get the response
        let response = self.client_base().sender().send_message(client_id, locked_session.get_chat_id(), voiceflow_message).await?;

        // Retrieve the last message sent by the bot from the response
        let bot_last_message = get_last_sent_message(&response);

        // Update the session with the previous message
        locked_session.set_previous_message(bot_last_message).await;

        // Return the response
        Ok(response)
    }

    /// Interacts with the client based on the provided update.
    ///
    /// This method determines the type of interaction (button press, text message or carousel switch),
    /// processes the interaction with the Voiceflow client, and updates the session state accordingly.
    ///
    /// **This method has a base implementation for sending messages. Modify it only if you
    /// know what you are doing or have devised a better approach.**
    ///
    /// # Parameters
    ///
    /// * `update` - The update from the client.
    /// * `update_state` - The optional state for updating the dialog.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing a vector of `SenderResponder` or a `VoiceflousionError` if the request fails.
    async fn interact_with_client(&self, update: Self::ClientUpdate<'_>, update_state: Option<State>) -> VoiceflousionResult<Vec<<Self::ClientSender<'_> as Sender>::SenderResponder>> {
        if !self.client_base().is_active(){
            return Err(VoiceflousionError::ClientRequestError(format!("Client {} is deactivated!", self.client_base().client_id()), "".to_string()))
        }
        // Get the interaction time from the update
        let interaction_time = update.interaction_time();

        // Check if a session exists for the given chat_id
        if let Some(telegram_session) = self.client_base().sessions().get_session(update.chat_id()).await {

            // Lock the session for safe access
            let locked_session = LockedSession::try_from_session(&telegram_session)?;

            // Check if the update is deprecated
            if let Some(message) = locked_session.previous_message().await.deref() {
                update.is_deprecated(message.date())?
            }

            // Handle the interaction based on its type
            match update.interaction_type() {
                // If it is a  regular button press
                InteractionType::Button(button_index) => {
                    // Handle the button interaction
                    self.choose_button_in_voiceflow_dialog(&locked_session, interaction_time, update_state, button_index.clone()).await
                },
                // If it is a text message
                InteractionType::Text(message) => {
                    // Handle the text message
                    self.send_message_to_voiceflow_dialog(&locked_session, interaction_time, message, update_state).await
                },
                // If it is a carousel switch button press
                InteractionType::CarouselSwitch(switch_direction) => {
                    // Handle carousel switch
                    self.handle_carousel_switch(&locked_session, interaction_time, switch_direction.clone()).await
                }

            }
        } else {

            // If no session exists, create a new session and launch the dialog
            let telegram_session = self.client_base().sessions().add_session(update.chat_id().clone()).await;
            let locked_session = LockedSession::try_from_session(&telegram_session)?;

            // Check if the update is deprecated
            if let Some(message) = locked_session.previous_message().await.deref() {
                 update.is_deprecated(message.date())?
            }

            self.launch_voiceflow_dialog(&locked_session, interaction_time).await
        }
    }


    /// Handles carousel switch interactions on the client.
    ///
    /// This method processes carousel switch interactions, sending the appropriate data to the Voiceflow client
    /// and handling the response.
    ///
    /// # Parameters
    ///
    /// * `locked_session` - The locked session for the interaction.
    /// * `interaction_time` - The time of the interaction.
    /// * `switch_direction` - Direction of the carousel switch (true for next, false for previous).
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing a vector of `SenderResponder` or a `VoiceflousionError` if the request fails.
    async fn handle_carousel_switch(&self, locked_session: &LockedSession<'_>, interaction_time: i64, switch_direction: bool) -> VoiceflousionResult<Vec<<Self::ClientSender<'_> as Sender>::SenderResponder>>;
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
    if let Some(responder) = response.last(){
        Some(responder.create_sent_message())
    }
    else{
        None
    }
}