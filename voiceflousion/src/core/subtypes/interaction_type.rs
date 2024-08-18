/// Represents different types of interactions in the integration.
///
/// `InteractionType` can be a text message, a button interaction with an index, or a carousel switch interaction.
#[derive(Debug)]
pub enum InteractionType {
    /// Represents a text interaction.
    Text(String),

    /// Represents a button interaction with the associated index.
    Button(usize, bool),

    /// Represents a carousel switch interaction with a direction (`true` for next, `false` for previous).
    CarouselSwitch(bool),
}

impl InteractionType {
    /// Creates a new `InteractionType` based on the provided parameters.
    ///
    /// # Parameters
    ///
    /// * `message` - The message associated with the interaction, used for text interactions.
    /// * `button_index` - An optional index for a button interaction.
    /// * `carousel_switch_direction` - An optional direction for a carousel switch interaction (`true` for next, `false` for previous).
    ///
    /// # Returns
    ///
    /// An instance of `InteractionType` representing the appropriate type of interaction: `Text`, `Button`, or `CarouselSwitch`.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::subtypes::InteractionType;
    ///
    /// let button_interaction = InteractionType::new("".to_string(), Some(0), None); // Button interaction
    /// let carousel_interaction = InteractionType::new("".to_string(), None, Some(true)); // Carousel switch interaction
    /// let text_interaction = InteractionType::new("Hello, world!".to_string(), None, None); // Text interaction
    /// ```
    pub fn new(message: String, button_options: Option<(usize, bool)>, carousel_switch_direction: Option<bool>) -> Self {
        match (button_options, carousel_switch_direction) {
            (_, Some(direction)) => InteractionType::CarouselSwitch(direction),
            (Some((index, is_url)), None) => InteractionType::Button(index, is_url),
            (None, None) => InteractionType::Text(message),
        }
    }
}