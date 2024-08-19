use crate::core::voiceflow::dialog_blocks::voiceflow_text::VoiceflowText;

/// Represents an option for Voiceflow buttons.
///
/// `VoiceflowButtonsOption` is an enum that can represent different options
/// such as associating text or being empty.
#[derive(Debug, Clone)]
pub enum VoiceflowButtonsOption {
    /// An option to associate text with the buttons.
    Text(VoiceflowText),

    /// An empty option, indicating no specific association.
    Empty,
}