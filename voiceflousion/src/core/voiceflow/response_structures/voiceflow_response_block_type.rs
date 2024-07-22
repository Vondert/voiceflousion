/// Represents the different types of blocks in a Voiceflow response.
///
/// `VoiceflowResponseBlockType` is used to categorize the various blocks that can be part
/// of a Voiceflow response.
#[derive(Debug)]
pub(crate) enum VoiceflowResponseBlockType {
    /// A text block.
    Text,

    /// A choice block.
    Choice,

    /// A cardV2 block.
    CardV2,

    /// A visual block.
    Visual,

    /// A carousel block.
    Carousel,

    /// An end block.
    End,

    /// A block type that is not recognized.
    None,
}
impl VoiceflowResponseBlockType {
    /// Creates a new `VoiceflowResponseBlockType` from a string representation.
    ///
    /// # Parameters
    ///
    /// * `response_type` - A string representing the type of the block.
    ///
    /// # Returns
    ///
    /// A `VoiceflowResponseBlockType` corresponding to the string representation.
    ///
    /// # Example
    ///
    /// ```
    /// let block_type = VoiceflowResponseBlockType::new("text");
    /// assert_eq!(block_type, VoiceflowResponseBlockType::Text);
    /// ```
    pub fn new(response_type: &str) -> VoiceflowResponseBlockType {
        match response_type {
            "text" => VoiceflowResponseBlockType::Text,
            "choice" => VoiceflowResponseBlockType::Choice,
            //"block" => VoiceflowResponseBlockType::Block,
            "cardV2" => VoiceflowResponseBlockType::CardV2,
            "visual" => VoiceflowResponseBlockType::Visual,
            "carousel" => VoiceflowResponseBlockType::Carousel,
            "end" => VoiceflowResponseBlockType::End,
            _ => VoiceflowResponseBlockType::None
        }
    }
}