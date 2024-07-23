/// Represents different types of interactions in the integration.
///
/// `InteractionType` can be a text message, a button interaction, or an undefined interaction.
#[derive(Debug)]
pub enum InteractionType {
    /// Represents a text interaction.
    Text(String),
    /// Represents a button interaction with the associated message and path.
    Button(String, String),
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
    /// let interaction = InteractionType::new("message".to_string(), Some("path".to_string()));
    /// let interaction = InteractionType::new("message".to_string(), None);
    /// ```
    pub fn new(message: String, button_path: Option<String>) -> Self {
        match button_path {
            Some(path) => InteractionType::Button(message, path),
            None => InteractionType::Text(message),
        }
    }
}
