use crate::core::voiceflow::dialog_blocks::{VoiceflowButtons, VoiceflowCard, VoiceflowCarousel, VoiceflowImage, VoiceflowText};

/// Represents a block in a Voiceflow dialog.
///
/// `VoiceflowBlock` is an enum that can represent various types of blocks
/// such as text, image, buttons, card, carousel, or an end block.
#[derive(Debug, Clone)]
pub enum VoiceflowBlock {
    /// A text block containing a message.
    Text(VoiceflowText),

    /// An image block.
    Image(VoiceflowImage),

    /// A buttons block containing buttons.
    Buttons(VoiceflowButtons),

    /// A card block containing card content.
    Card(VoiceflowCard),

    /// A carousel block containing multiple cards.
    Carousel(VoiceflowCarousel),

    /// An end block indicating the end of the dialog.
    End,
}