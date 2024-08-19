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

impl VoiceflowText {
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

    /// Creates a `VoiceflowText` instance with a default error message.
    ///
    /// # Parameters
    ///
    /// * `error_text` - The error message string.
    ///
    /// # Returns
    ///
    /// A new instance of `VoiceflowText` containing the error message.
    pub(crate) fn error_default(error_text: &str) -> Self {
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
    pub fn message(&self) -> &String {
        &self.message
    }
}

impl FromValue for VoiceflowText {
    /// Attempts to convert a JSON `Value` into a `VoiceflowText` instance.
    ///
    /// This method extracts the "message" field from the JSON value, ensuring it is a string.
    /// If the message is empty or the conversion fails, it returns an error or `None`.
    ///
    /// # Parameters
    ///
    /// * `value` - A reference to the JSON `Value` to convert from.
    ///
    /// # Returns
    ///
    /// A `VoiceflousionResult` containing an `Option` with the `VoiceflowText` instance if the conversion
    /// succeeds, or a `VoiceflousionError` if the conversion fails. If the conversion
    /// succeeds but there is no meaningful value, `None` can be returned.
    fn from_value(value: &Value) -> VoiceflousionResult<Option<Self>> {
        // Extract the "message" field from the "payload" within the "trace" object in the JSON value.
        let message = value["trace"]["payload"].get("message")
            .and_then(|message| message.as_str())
            .map(|s| s.trim_matches('"').to_string())  // Remove surrounding quotes if present
            .ok_or_else(|| VoiceflousionError::VoiceflowBlockConvertationError(
                "VoiceflowText text message".to_string(),
                value.clone()
            ))?;

        // If the extracted message is empty, return None.
        if message.is_empty() {
            return Ok(None);
        }

        // Return the constructed `VoiceflowText` instance.
        Ok(Some(Self::new(message)))
    }
}
