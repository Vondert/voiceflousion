use serde_json::Value;

/// Represents different types of interactions in the integration.
///
/// `InteractionType` can be a text message, a button interaction, or an undefined interaction.
#[derive(Debug)]
pub enum InteractionType {
    /// Represents a text interaction.
    Text(String),
    /// Represents a button interaction with the associated path and payload.
    Button(Value),
    /// Represents an undefined interaction.
    Undefined(String),
}

impl InteractionType {
    /// Creates a new `InteractionType`.
    ///
    /// # Parameters
    ///
    /// * `message` - The message associated with the interaction.
    /// * `button_path` - An optional path for a button interaction.
    /// * `button_payload` - An optional payload for a button interaction.
    ///
    /// # Returns
    ///
    /// An instance of `InteractionType` representing the appropriate type of interaction.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::subtypes::InteractionType;
    ///
    /// let interaction = InteractionType::new("message".to_string(), Some(serde_json::json!({"key": "value"})));
    /// let interaction = InteractionType::new("message".to_string(), None);
    /// ```
    pub fn new(message: String, button_payload: Option<Value>) -> Self {
        match button_payload {
            Some(payload) => InteractionType::Button(payload),
            _ => InteractionType::Text(message),
        }
    }
}
