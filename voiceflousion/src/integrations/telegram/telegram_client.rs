use std::ops::Deref;
use async_trait::async_trait;
use crate::core::base_structs::ClientBase;
use crate::core::ClientBuilder;
use crate::core::session_wrappers::LockedSession;
use crate::core::traits::{Client, Sender};
use crate::core::voiceflow::VoiceflowBlock;
use crate::core::voiceflow::dialog_blocks::VoiceflowCarousel;
use crate::errors::{VoiceflousionError, VoiceflousionResult};
use crate::integrations::telegram::{TelegramResponder, TelegramSender, TelegramUpdate};

/// Represents a client for Telegram integration with Voiceflow.
///
/// `TelegramClient` manages sessions and interactions with the Voiceflow API and Telegram.
pub struct TelegramClient {
    /// The base structure that provides core functionalities.
    client_base: ClientBase<TelegramSender>,
}

impl TelegramClient {
    /// Creates a new `TelegramClient`.
    ///
    /// This method initializes a new `TelegramClient` using the provided `ClientBuilder`.
    /// It configures the client with the necessary parameters and returns an instance of `TelegramClient`.
    ///
    /// # Parameters
    ///
    /// * `builder` - The `ClientBuilder` containing the necessary configurations.
    ///
    /// # Returns
    ///
    /// A new instance of `TelegramClient`.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::ClientBuilder;
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    /// use voiceflousion::integrations::telegram::TelegramClient;
    ///
    /// let voiceflow_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, Some(120)));
    /// let builder = ClientBuilder::new("client_id".to_string(), "api_key".to_string(), voiceflow_client, 10);
    /// let client = TelegramClient::new(builder);
    /// ```
    pub fn new(builder: ClientBuilder) -> Self {
        let api_key = builder.api_key().clone();
        let max_connections_per_moment = builder.max_connections_per_moment();
        let connection_duration = builder.connection_duration();
        let sender = TelegramSender::new(max_connections_per_moment, api_key, connection_duration);

        Self {
            client_base: ClientBase::new(builder, sender),
        }
    }

    /// Switches the carousel card in the client's message.
    ///
    /// This method is used to switch between carousel cards in a Telegram session.
    /// It updates the session with the new carousel state and sends the updated carousel card.
    ///
    /// # Parameters
    ///
    /// * `locked_session` - The locked session for the interaction.
    /// * `carousel` - The `VoiceflowCarousel` block containing the cards.
    /// * `message_id` - The ID of the message containing the carousel.
    /// * `direction` - The direction to switch the carousel (`true` for next, `false` for previous).
    /// * `interaction_time` - The interaction time as a Unix timestamp.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing the `TelegramResponder` if successful, or a `VoiceflousionError` if the request fails.
    async fn switch_carousel_card(&self, locked_session: &LockedSession<'_>, carousel: &VoiceflowCarousel, message_id: &String, direction: bool, interaction_time: i64) -> VoiceflousionResult<TelegramResponder> {
        // Update the last interaction time in the session
        locked_session.set_last_interaction(Some(interaction_time));
        // Update the carousel with the new card and send the response
        self.client_base.sender().update_carousel(carousel, direction, locked_session.get_chat_id(), message_id).await
    }
}

#[async_trait]
impl Client for TelegramClient {
    type ClientUpdate<'async_trait> = TelegramUpdate;
    type ClientSender<'async_trait> = TelegramSender;

    /// Returns a reference to the `ClientBase`.
    ///
    /// # Returns
    ///
    /// A reference to the `ClientBase` instance.
    fn client_base(&self) -> &ClientBase<Self::ClientSender<'_>> {
        &self.client_base
    }

    /// Handles carousel switch interactions in a Telegram session.
    ///
    /// This method checks if the previous message contains a carousel block and, if so,
    /// switches the carousel card according to the specified direction. If the previous
    /// message is not a carousel, it returns an error indicating that there is no carousel to switch.
    ///
    /// # Parameters
    ///
    /// * `locked_session` - The locked session for the interaction, ensuring thread-safe access.
    /// * `interaction_time` - The time of the interaction in seconds since the Unix epoch.
    /// * `switch_direction` - The direction to switch the carousel (`true` for next, `false` for previous).
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing a vector of `SenderResponder` if the switch was successful,
    /// or a `VoiceflousionError` if the operation fails (e.g., no carousel to switch).
    async fn handle_carousel_switch(&self, locked_session: &LockedSession<'_>, interaction_time: i64, switch_direction: bool) -> VoiceflousionResult<Vec<<Self::ClientSender<'_> as Sender>::SenderResponder>> {
        let binding = locked_session.previous_message().await;
        let previous_message = binding.deref().as_ref().ok_or_else(|| {
            VoiceflousionError::ClientRequestError("TelegramClient".to_string(), "Carousel cannot be switched at the start of the conversation".to_string(), )
        })?;

        if let VoiceflowBlock::Carousel(carousel) = previous_message.block() {
            Ok(vec![self.switch_carousel_card(locked_session, carousel, previous_message.id(), switch_direction, interaction_time).await?])
        } else {
            Err(VoiceflousionError::ValidationError(
                "TelegramClient".to_string(),
                "There is no carousel to switch".to_string(),
            ))
        }
    }
}