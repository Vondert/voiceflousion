/// Represents different types of interactions in the integration.
///
/// `InteractionType` can be a text message, a button interaction with an index, or an undefined interaction.
#[derive(Debug)]
pub enum InteractionType {
    /// Represents a text interaction.
    Text(String),

    /// Represents a button interaction with the associated index.
    Button(i64),

    /// Represents an undefined interaction with the associated data.
    Undefined(String),
}

impl InteractionType {
    /// Creates a new `InteractionType`.
    ///
    /// # Parameters
    ///
    /// * `message` - The message associated with the interaction, used for text or undefined interactions.
    /// * `button_index` - An optional index for a button interaction.
    ///
    /// # Returns
    ///
    /// An instance of `InteractionType` representing the appropriate type of interaction, either `Text`, `Button`, or `Undefined`.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::subtypes::InteractionType;
    ///
    /// let interaction = InteractionType::new("Hello, world!".to_string(), Some(0)); // Button interaction
    /// let interaction = InteractionType::new("Hello, world!".to_string(), None); // Text interaction
    /// ```
    pub fn new(message: String, button_index: Option<i64>) -> Self {
        match button_index {
            Some(index) => InteractionType::Button(index),
            None => InteractionType::Text(message),
        }
    }
}