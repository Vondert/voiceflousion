use crate::voiceflow::dialog_blocks::{VoiceflowButtons, VoiceflowCard, VoiceflowCarousel, VoiceflowImage, VoiceflowText};

#[derive(Debug)]
pub enum VoiceflowBlock{
    Text(VoiceflowText),
    Image(VoiceflowImage),
    Buttons(VoiceflowButtons),
    Card(VoiceflowCard),
    Carousel(VoiceflowCarousel)
}