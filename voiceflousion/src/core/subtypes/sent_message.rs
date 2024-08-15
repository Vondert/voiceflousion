use serde_json::Value;
use crate::core::voiceflow::VoiceflowBlock;
use crate::errors::{VoiceflousionError, VoiceflousionResult};

/// Represents a sent message in the integration.
///
/// `SentMessage` holds details of a message sent, including the associated block, message ID, and date.
pub struct SentMessage {
    /// The block associated with the sent message.
    block: VoiceflowBlock,

    /// The ID of the sent message.
    message_id: String,

    /// The date the message was sent, represented as a Unix timestamp.
    date: i64,
}

impl SentMessage {
    /// Creates a new `SentMessage`.
    ///
    /// # Parameters
    ///
    /// * `block` - The `VoiceflowBlock` associated with the message.
    /// * `message_id` - The ID of the message.
    /// * `date` - The date the message was sent, represented as a Unix timestamp.
    ///
    /// # Returns
    ///
    /// A new instance of `SentMessage`.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::subtypes::SentMessage;
    /// use voiceflousion::core::voiceflow::dialog_blocks::VoiceflowText;
    /// use voiceflousion::core::voiceflow::VoiceflowBlock;
    ///
    /// let block = VoiceflowBlock::Text(VoiceflowText::new("text".to_string()));
    /// let sent_message = SentMessage::new(block, "message_id".to_string(), 1627554661);
    /// ```
    pub fn new(block: VoiceflowBlock, message_id: String, date: i64) -> Self {
        Self {
            block,
            message_id,
            date,
        }
    }

    /// Returns the ID of the sent message.
    ///
    /// # Returns
    ///
    /// A reference to the message ID string.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::subtypes::SentMessage;
    /// use voiceflousion::core::voiceflow::dialog_blocks::VoiceflowText;
    /// use voiceflousion::core::voiceflow::VoiceflowBlock;
    ///
    /// let block = VoiceflowBlock::Text(VoiceflowText::new("text".to_string()));
    /// let sent_message = SentMessage::new(block, "message_id".to_string(), 1627554661);
    ///
    /// let id = sent_message.id();
    /// ```
    pub fn id(&self) -> &String {
        &self.message_id
    }

    /// Returns the block associated with the sent message.
    ///
    /// # Returns
    ///
    /// A reference to the `VoiceflowBlock`.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::subtypes::SentMessage;
    /// use voiceflousion::core::voiceflow::dialog_blocks::VoiceflowText;
    /// use voiceflousion::core::voiceflow::VoiceflowBlock;
    ///
    /// let block = VoiceflowBlock::Text(VoiceflowText::new("text".to_string()));
    /// let sent_message = SentMessage::new(block, "message_id".to_string(), 1627554661);
    ///
    /// let block = sent_message.block();
    /// ```
    pub fn block(&self) -> &VoiceflowBlock {
        &self.block
    }

    /// Returns the date the message was sent.
    ///
    /// # Returns
    ///
    /// An `i64` representing the date the message was sent as a Unix timestamp.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::subtypes::SentMessage;
    /// use voiceflousion::core::voiceflow::dialog_blocks::VoiceflowText;
    /// use voiceflousion::core::voiceflow::VoiceflowBlock;
    ///
    /// let block = VoiceflowBlock::Text(VoiceflowText::new("text".to_string()));
    /// let sent_message = SentMessage::new(block, "message_id".to_string(), 1627554661);
    ///
    /// let date = sent_message.date();
    /// ```
    pub fn date(&self) -> i64 {
        self.date
    }

    /// Retrieves the payload of a button by its index from the associated block.
    ///
    /// This method attempts to extract the payload of a button from the `VoiceflowBlock` associated with the sent message.
    /// It supports blocks that contain buttons, such as `Buttons`, `Card`, and `Carousel`.
    ///
    /// # Parameters
    ///
    /// * `button_index` - The index of the button within the block to retrieve the payload from.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing the button's payload as a `Value`, or an error if the block does not contain buttons
    /// or the button index is out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// use serde_json::json;
    /// use voiceflousion::core::subtypes::SentMessage;
    /// use voiceflousion::core::voiceflow::dialog_blocks::{VoiceflowButton, VoiceflowButtons};
    /// use voiceflousion::core::voiceflow::dialog_blocks::enums::VoiceflowButtonActionType;
    /// use voiceflousion::core::voiceflow::VoiceflowBlock;
    ///
    /// let button = VoiceflowButton::new("Click me".to_string(), VoiceflowButtonActionType::Path, json!("payload"));
    /// let buttons = VoiceflowButtons::new(vec![button]);
    /// let block = VoiceflowBlock::Buttons(buttons);
    /// let sent_message = SentMessage::new(block, "message_id".to_string(), 1627554661);
    ///
    /// let payload = sent_message.get_button_payload(0).unwrap();
    /// ```
    pub fn get_button_payload(&self, button_index: usize) -> VoiceflousionResult<Value> {
        match &self.block {
            VoiceflowBlock::Buttons(buttons) => {
                let button = buttons.get_button(button_index)?;
                Ok(button.payload().clone())
            },
            VoiceflowBlock::Card(card) => {
                let buttons = card.buttons().as_ref().unwrap();
                let button = buttons.get_button(button_index)?;
                Ok(button.payload().clone())
            },
            VoiceflowBlock::Carousel(carousel) => {
                let (card, _index) = carousel.get_selected_card()?;
                let buttons = card.buttons().as_ref().unwrap();
                let button = buttons.get_button(button_index)?;
                Ok(button.payload().clone())
            },
            _ => Err(VoiceflousionError::ValidationError("SentMessage content".to_string(), "There are no buttons in the previous message!".to_string()))
        }
    }
}
