use std::fmt::Debug;
use crate::voiceflow::dialog_blocks::enums::{VoiceflowBlock, VoiceflowButtonsOption};
use crate::voiceflow::dialog_blocks::traits::FromValue;
use crate::voiceflow::dialog_blocks::{VoiceflowButtons, VoiceflowCard, VoiceflowCarousel, VoiceflowImage, VoiceflowText};
use crate::voiceflow::response_structures::{VoiceflowResponseBlock, VoiceflowResponseBlockType};
use crate::voiceflow::VoiceflowError;

#[derive(Debug)]
pub struct VoiceflowMessage{
    content: Vec<VoiceflowBlock>
}
impl VoiceflowMessage{
    pub fn add_block(&mut self, block: VoiceflowBlock) -> (){
        self.content.push(block);
    }
}
impl Default for VoiceflowMessage{
    fn default() -> Self {
        Self{
            content: Vec::new()
        }
    }
}
pub(crate) struct VoiceflowMessageBuilder;
impl VoiceflowMessageBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build_message(self, blocks: Vec<VoiceflowResponseBlock>) -> Result<VoiceflowMessage, VoiceflowError> {
        let mut message = VoiceflowMessage {
            content: Vec::with_capacity(blocks.len()),
        };
        let mut buttons_options = VoiceflowButtonsOption::Empty;

        for block in blocks {
            if let VoiceflowButtonsOption::Empty = buttons_options {
                match block.block_type() {
                    VoiceflowResponseBlockType::Text => {
                        let text = VoiceflowText::from_value(block.json())?;
                        buttons_options = VoiceflowButtonsOption::Text(text);
                    },
                    VoiceflowResponseBlockType::Choice => {
                        let buttons = VoiceflowBlock::Buttons(VoiceflowButtons::from_value(block.json())?);
                        message.content.push(buttons);
                    },
                    VoiceflowResponseBlockType::CardV2 => {
                        let card = VoiceflowBlock::Card(VoiceflowCard::from_value(block.json())?);
                        message.content.push(card);
                    },
                    VoiceflowResponseBlockType::Visual => {
                        let image = VoiceflowImage::from_value(block.json())?;
                        buttons_options = VoiceflowButtonsOption::Image(image);
                    },
                    VoiceflowResponseBlockType::Carousel => {
                        let carousel = VoiceflowBlock::Carousel(VoiceflowCarousel::from_value(block.json())?);
                        message.content.push(carousel);
                    },
                    _ => {},
                }
            } else {
                match block.block_type() {
                    VoiceflowResponseBlockType::Choice => {
                        let mut buttons = VoiceflowButtons::from_value(block.json())?;
                        buttons.set_option(buttons_options);
                        message.content.push(VoiceflowBlock::Buttons(buttons));
                        buttons_options = VoiceflowButtonsOption::Empty;
                    },
                    _ => {
                        match buttons_options {
                            VoiceflowButtonsOption::Text(text) => message.content.push(VoiceflowBlock::Text(text)),
                            VoiceflowButtonsOption::Image(image) => message.content.push(VoiceflowBlock::Image(image)),
                            _ => {},
                        }
                        buttons_options = VoiceflowButtonsOption::Empty;

                        match block.block_type() {
                            VoiceflowResponseBlockType::Text => {
                                let text = VoiceflowText::from_value(block.json())?;
                                buttons_options = VoiceflowButtonsOption::Text(text);
                            },
                            VoiceflowResponseBlockType::Visual => {
                                let image = VoiceflowImage::from_value(block.json())?;
                                buttons_options = VoiceflowButtonsOption::Image(image);
                            },
                            VoiceflowResponseBlockType::CardV2 => {
                                let card = VoiceflowBlock::Card(VoiceflowCard::from_value(block.json())?);
                                message.content.push(card);
                            },
                            VoiceflowResponseBlockType::Carousel => {
                                let carousel = VoiceflowBlock::Carousel(VoiceflowCarousel::from_value(block.json())?);
                                message.content.push(carousel);
                            },
                            _ => {},
                        }
                    },
                }
            }
        }

        match buttons_options {
            VoiceflowButtonsOption::Text(text) => message.content.push(VoiceflowBlock::Text(text)),
            VoiceflowButtonsOption::Image(image) => message.content.push(VoiceflowBlock::Image(image)),
            _ => {},
        }

        Ok(message)
    }
}
