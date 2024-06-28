use crate::voiceflow::dialog_blocks::traits::FromValue;
use crate::voiceflow::dialog_blocks::voiceflow_buttons::VoiceflowButtons;
use crate::voiceflow::dialog_blocks::voiceflow_card::VoiceflowCard;
use crate::voiceflow::dialog_blocks::voiceflow_carousel::VoiceflowCarousel;
use crate::voiceflow::dialog_blocks::voiceflow_image::VoiceflowImage;
use crate::voiceflow::dialog_blocks::voiceflow_text::VoiceflowText;
use crate::voiceflow::response_structures::{VoiceflowResponseBlock, VoiceflowResponseBlockType};
use crate::voiceflow::VoiceflowError;

#[derive(Debug)]
pub(crate) struct VoiceflowMessage{
    text: Vec<VoiceflowText>,
    image: Vec<VoiceflowImage>,
    card: Vec<VoiceflowCard>,
    carousel: Vec<VoiceflowCarousel>,
    buttons: Option<VoiceflowButtons>
}
pub(crate) struct VoiceflowMessageBuilder;
impl VoiceflowMessageBuilder{
    pub fn new() -> Self{
        Self
    }
    pub fn build_message(self, blocks: Vec<VoiceflowResponseBlock>) -> Result<VoiceflowMessage, VoiceflowError>{
        let mut message = VoiceflowMessage{
            text: vec![],
            image: vec![],
            card: vec![],
            carousel: vec![],
            buttons: None,
        };
        for block in blocks{
            match block.block_type{
                VoiceflowResponseBlockType::Text => {
                    message.text.push(VoiceflowText::from_value(block.json)?);
                },
                VoiceflowResponseBlockType::Choice => {
                    message.buttons = Some(VoiceflowButtons::from_value(block.json)?);
                },
                VoiceflowResponseBlockType::CardV2 => {
                    message.card.push(VoiceflowCard::from_value(block.json)?);
                },
                VoiceflowResponseBlockType::Visual => {
                    message.image.push(VoiceflowImage::from_value(block.json)?);
                },
                VoiceflowResponseBlockType::Carousel => {
                    message.carousel.push(VoiceflowCarousel::from_value(block.json)?);
                },
                _ => {}
            }
        }
        Ok(message)
    }
}
