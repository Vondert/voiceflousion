use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::vec::IntoIter;
use crate::voiceflow::dialog_blocks::enums::VoiceflowButtonsOption;
use crate::voiceflow::dialog_blocks::{VoiceflowButtons, VoiceflowCard, VoiceflowCarousel, VoiceflowImage, VoiceflowText};
use crate::voiceflow::response_structures::{VoiceflowResponseBlock, VoiceflowResponseBlockType};
use crate::voiceflow::{VoiceflowBlock, VoiceflousionError};
use crate::voiceflow::dialog_blocks::traits::FromValue;

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
impl Deref for VoiceflowMessage{
    type Target = Vec<VoiceflowBlock>;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}
impl DerefMut for VoiceflowMessage{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}
impl IntoIterator for VoiceflowMessage {
    type Item = VoiceflowBlock;
    type IntoIter = IntoIter<VoiceflowBlock>;

    fn into_iter(self) -> Self::IntoIter {
        self.content.into_iter()
    }
}
pub(super) struct VoiceflowMessageBuilder;
impl VoiceflowMessageBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build_message(self, blocks: Vec<VoiceflowResponseBlock>) -> Result<VoiceflowMessage, VoiceflousionError> {
        let mut message = VoiceflowMessage {
            content: Vec::with_capacity(blocks.len()),
        };
        let mut buttons_options = VoiceflowButtonsOption::Empty;

        for block in blocks {
            if let VoiceflowButtonsOption::Empty = buttons_options {
                match block.block_type() {
                    VoiceflowResponseBlockType::Text => {
                        if let Some(text) = VoiceflowText::from_value(block.json())?{
                            buttons_options = VoiceflowButtonsOption::Text(text);
                        }
                    },
                    VoiceflowResponseBlockType::Choice => {
                        if let Some(buttons) = VoiceflowButtons::from_value(block.json())?{
                            message.content.push(VoiceflowBlock::Buttons(buttons));
                        }
                    },
                    VoiceflowResponseBlockType::CardV2 => {
                        if let Some(card) = VoiceflowCard::from_value(block.json())?{
                            message.content.push(VoiceflowBlock::Card(card));
                        }
                    },
                    VoiceflowResponseBlockType::Visual => {
                        if let Some(image) = VoiceflowImage::from_value(block.json())?{
                            buttons_options = VoiceflowButtonsOption::Image(image);
                        }
                    },
                    VoiceflowResponseBlockType::Carousel => {
                        if let Some(carousel) = VoiceflowCarousel::from_value(block.json())?{
                            message.content.push(VoiceflowBlock::Carousel(carousel));
                        }
                    },
                    _ => {},
                }
            } else {
                match block.block_type() {
                    VoiceflowResponseBlockType::Choice => {
                        if let Some(mut buttons) = VoiceflowButtons::from_value(block.json())?{
                            buttons.set_option(buttons_options);
                            message.content.push(VoiceflowBlock::Buttons(buttons));
                            buttons_options = VoiceflowButtonsOption::Empty;
                        }
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
                                if let Some(text) = VoiceflowText::from_value(block.json())?{
                                    buttons_options = VoiceflowButtonsOption::Text(text);
                                }
                            },
                            VoiceflowResponseBlockType::CardV2 => {
                                if let Some(card) = VoiceflowCard::from_value(block.json())?{
                                    message.content.push(VoiceflowBlock::Card(card));
                                }
                            },
                            VoiceflowResponseBlockType::Visual => {
                                if let Some(image) = VoiceflowImage::from_value(block.json())?{
                                    buttons_options = VoiceflowButtonsOption::Image(image);
                                }
                            },
                            VoiceflowResponseBlockType::Carousel => {
                                if let Some(carousel) = VoiceflowCarousel::from_value(block.json())?{
                                    message.content.push(VoiceflowBlock::Carousel(carousel));
                                }
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
