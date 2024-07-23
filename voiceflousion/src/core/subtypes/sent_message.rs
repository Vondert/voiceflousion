use crate::core::voiceflow::VoiceflowBlock;

/// Represents a sent message in the integration.
///
/// `SentMessage` holds details of a message sent, including the block, message ID, and date.
pub struct SentMessage {
    /// The block associated with the sent message.
    block: VoiceflowBlock,
    /// The ID of the sent message.
    message_id: String,
    /// The date the message was sent.
    date: i64,
}

impl SentMessage {
    /// Creates a new `SentMessage`.
    ///
    /// # Parameters
    ///
    /// * `block` - The `VoiceflowBlock` associated with the message.
    /// * `message_id` - The ID of the message.
    /// * `date` - The date the message was sent.
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
    /// An `i64` representing the date the message was sent.
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
}
