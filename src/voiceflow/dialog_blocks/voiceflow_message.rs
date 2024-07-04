use std::fmt::Debug;
use crate::voiceflow::dialog_blocks::traits::{FromValue, VoiceflowBlock};
use crate::voiceflow::dialog_blocks::voiceflow_buttons::{VoiceflowButtons, VoiceflowButtonsOption};
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
                        let buttons = Box::new(VoiceflowButtons::from_value(block.json())?);
                        message.content.push(buttons);
                    },
                    VoiceflowResponseBlockType::CardV2 => {
                        let card = Box::new(VoiceflowCard::from_value(block.json())?);
                        message.content.push(card);
                    },
                    VoiceflowResponseBlockType::Visual => {
                        let image = VoiceflowImage::from_value(block.json())?;
                        buttons_options = VoiceflowButtonsOption::Image(image);
                    },
                    VoiceflowResponseBlockType::Carousel => {
                        let carousel = Box::new(VoiceflowCarousel::from_value(block.json())?);
                        message.content.push(carousel);
                    },
                    _ => {},
                }
            } else {
                match block.block_type() {
                    VoiceflowResponseBlockType::Choice => {
                        let mut buttons = Box::new(VoiceflowButtons::from_value(block.json())?);
                        buttons.set_option(buttons_options);
                        message.content.push(buttons);
                        buttons_options = VoiceflowButtonsOption::Empty;
                    },
                    _ => {
                        match buttons_options {
                            VoiceflowButtonsOption::Text(text) => message.content.push(Box::new(text)),
                            VoiceflowButtonsOption::Image(image) => message.content.push(Box::new(image)),
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
                                let card = Box::new(VoiceflowCard::from_value(block.json())?);
                                message.content.push(card);
                            },
                            VoiceflowResponseBlockType::Carousel => {
                                let carousel = Box::new(VoiceflowCarousel::from_value(block.json())?);
                                message.content.push(carousel);
                            },
                            _ => {},
                        }
                    },
                }
            }
        }

        match buttons_options {
            VoiceflowButtonsOption::Text(text) => message.content.push(Box::new(text)),
            VoiceflowButtonsOption::Image(image) => message.content.push(Box::new(image)),
            _ => {},
        }

        Ok(message)
    }
}
