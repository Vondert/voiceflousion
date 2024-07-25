use crate::core::voiceflow::VoiceflowBlock;

/// `ResponderBase` is the foundational struct for managing responses within the system.
///
/// This struct encapsulates essential components such as the message ID, date of the message,
/// and the content of the message.
#[derive(Debug)]
pub struct ResponderBase{
    /// The ID of the message.
    message_id: String,
    /// The date when the message was sent.
    date: i64,
    /// The content of the message.
    message_content: VoiceflowBlock
}
impl ResponderBase{
    /// Creates a new `ResponderBase`.
    ///
    /// # Parameters
    ///
    /// * `message_id` - The ID of the message.
    /// * `message_content` - The content of the message.
    /// * `date` - The date when the message was sent.
    ///
    /// # Returns
    ///
    /// A new instance of `ResponderBase`.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::base_structs::ResponderBase;
    /// use voiceflousion::core::voiceflow::dialog_blocks::VoiceflowText;
    /// use voiceflousion::core::voiceflow::VoiceflowBlock;
    ///
    /// let block = VoiceflowBlock::Text(VoiceflowText::new("text".to_string()));
    /// let responder_base = ResponderBase::new("message_id".to_string(), block, 1627554661);
    /// ```
    pub fn new(message_id: String, message_content: VoiceflowBlock, date: i64) -> Self {
        Self {
            message_id,
            date,
            message_content,
        }
    }

    /// Returns a reference to the message ID.
    ///
    /// # Returns
    ///
    /// A reference to the message ID string.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::base_structs::ResponderBase;
    /// use voiceflousion::core::voiceflow::dialog_blocks::VoiceflowText;
    /// use voiceflousion::core::voiceflow::VoiceflowBlock;
    ///
    /// let block = VoiceflowBlock::Text(VoiceflowText::new("text".to_string()));
    /// let responder_base = ResponderBase::new("message_id".to_string(), block, 1627554661);
    /// let message_id = responder_base.message_id();
    /// ```
    pub fn message_id(&self) -> &String {
        &self.message_id
    }

    /// Returns a reference to the content of the message.
    ///
    /// # Returns
    ///
    /// A reference to the `VoiceflowBlock` representing the message content.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::base_structs::ResponderBase;
    /// use voiceflousion::core::voiceflow::dialog_blocks::VoiceflowText;
    /// use voiceflousion::core::voiceflow::VoiceflowBlock;
    ///
    /// let block = VoiceflowBlock::Text(VoiceflowText::new("text".to_string()));
    /// let responder_base = ResponderBase::new("message_id".to_string(), block, 1627554661);
    /// let content = responder_base.message_content();
    /// ```
    pub fn message_content(&self) -> &VoiceflowBlock {
        &self.message_content
    }

    /// Returns the date of the message.
    ///
    /// # Returns
    ///
    /// The date of the message as an `i64` timestamp.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::base_structs::ResponderBase;
    /// use voiceflousion::core::voiceflow::dialog_blocks::VoiceflowText;
    /// use voiceflousion::core::voiceflow::VoiceflowBlock;
    ///
    /// let block = VoiceflowBlock::Text(VoiceflowText::new("text".to_string()));
    /// let responder_base = ResponderBase::new("message_id".to_string(), block, 1627554661);
    /// let date = responder_base.date();
    /// ```
    pub fn date(&self) -> i64 {
        self.date
    }
}