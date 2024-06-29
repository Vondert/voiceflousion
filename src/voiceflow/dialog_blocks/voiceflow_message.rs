use std::fmt::Debug;
use crate::voiceflow::dialog_blocks::traits::{FromValue, VoiceflowBlock};
use crate::voiceflow::dialog_blocks::voiceflow_buttons::VoiceflowButtons;
use crate::voiceflow::dialog_blocks::voiceflow_card::VoiceflowCard;
use crate::voiceflow::dialog_blocks::voiceflow_carousel::VoiceflowCarousel;
use crate::voiceflow::dialog_blocks::voiceflow_image::VoiceflowImage;
use crate::voiceflow::dialog_blocks::voiceflow_text::VoiceflowText;
use crate::voiceflow::response_structures::{VoiceflowResponseBlock, VoiceflowResponseBlockType};
use crate::voiceflow::VoiceflowError;

#[derive(Debug)]
pub(crate) struct VoiceflowMessage{
    content: Vec<Box<dyn VoiceflowBlock>>
}
pub(crate) struct VoiceflowMessageBuilder;
impl VoiceflowMessageBuilder{
    pub fn new() -> Self{
        Self
    }
    pub fn build_message(self, blocks: Vec<VoiceflowResponseBlock>) -> Result<VoiceflowMessage, VoiceflowError>{
        let mut message = VoiceflowMessage{
            content: Vec::with_capacity(blocks.len()),
        };
        for block in blocks{
            match block.block_type{
                VoiceflowResponseBlockType::Text => {
                    let text = Box::new(VoiceflowText::from_value(block.json)?);
                    message.content.push(text);
                },
                VoiceflowResponseBlockType::Choice => {
                    let buttons = Box::new(VoiceflowButtons::from_value(block.json)?);
                    message.content.push(buttons);
                },
                VoiceflowResponseBlockType::CardV2 => {
                    let card = Box::new(VoiceflowCard::from_value(block.json)?);
                    message.content.push(card);
                },
                VoiceflowResponseBlockType::Visual => {
                    let image = Box::new(VoiceflowImage::from_value(block.json)?);
                    message.content.push(image);
                },
                VoiceflowResponseBlockType::Carousel => {
                    let carousel = Box::new(VoiceflowImage::from_value(block.json)?);
                    message.content.push(carousel);
                },
                _ => {}
            }
        }
        Ok(message)
    }
}
