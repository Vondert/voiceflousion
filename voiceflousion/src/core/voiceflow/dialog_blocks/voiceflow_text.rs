use serde_json::Value;
use crate::core::voiceflow::dialog_blocks::traits::FromValue;
use crate::errors::{VoiceflousionError, VoiceflousionResult};

/// Represents a text message block in a Voiceflow response.
///
/// `VoiceflowText` contains the message string extracted from a Voiceflow response.
#[derive(Debug, Clone)]
pub struct VoiceflowText {
    /// The message string of the text block.
    message: String,
}
impl VoiceflowText{
    /// Creates a new `VoiceflowText` instance.
    ///
    /// # Parameters
    ///
    /// * `message` - The message string of the text block.
    ///
    /// # Returns
    ///
    /// A new instance of `VoiceflowText`.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::voiceflow::dialog_blocks::VoiceflowText;
    ///
    /// let text_block = VoiceflowText::new("Hello, World!".to_string());
    /// ```
    pub fn new(message: String) -> Self {
        Self { message }
    }

    pub(crate) fn error_default(error_text: &str) -> Self{
        Self::new(error_text.to_string())
    }

    /// Returns a reference to the message string of the text block.
    ///
    /// # Returns
    ///
    /// A reference to the message string.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::voiceflow::dialog_blocks::VoiceflowText;
    ///
    /// let text_block = VoiceflowText::new("Hello, World!".to_string());
    /// let message = text_block.message();
    /// ```
    pub fn message(&self) -> &String{
        &self.message
    }
}

impl FromValue for VoiceflowText{
    fn from_value(value: &Value) -> VoiceflousionResult<Option<Self>> {
        let message = value["trace"]["payload"].get("message")
            .ok_or_else(|| VoiceflousionError::VoiceflowBlockConvertationError(("VoiceflowText text message".to_string(), value.clone())))?
            .as_str()
            .map(|s| s.trim_matches('"').to_string())
            .ok_or_else(|| VoiceflousionError::VoiceflowBlockConvertationError(("VoiceflowText text message".to_string(), value.clone())))?;

        if message.is_empty(){
            return Ok(None)
        }
        Ok(Some(Self::new(message)))
    }
}