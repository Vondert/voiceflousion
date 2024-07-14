use crate::voiceflow::dialog_blocks::voiceflow_image::VoiceflowImage;
use crate::voiceflow::dialog_blocks::voiceflow_text::VoiceflowText;

#[derive(Debug, Clone)]
pub enum VoiceflowButtonsOption{
    Text(VoiceflowText),
    Image(VoiceflowImage),
    Empty
}