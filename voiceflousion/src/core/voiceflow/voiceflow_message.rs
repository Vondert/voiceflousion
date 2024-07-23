use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::vec::IntoIter;
use crate::core::voiceflow::dialog_blocks::enums::VoiceflowButtonsOption;
use crate::core::voiceflow::dialog_blocks::{VoiceflowButtons, VoiceflowCard, VoiceflowCarousel, VoiceflowImage, VoiceflowText};
use crate::core::voiceflow::response_structures::{VoiceflowResponseBlock, VoiceflowResponseBlockType};
use crate::core::voiceflow::{VoiceflowBlock, VoiceflousionError};
use crate::core::voiceflow::dialog_blocks::traits::FromValue;

/// Represents a message from a Voiceflow response.
///
/// `VoiceflowMessage` contains a list of `VoiceflowBlock` instances that make up the content of the message.
#[derive(Debug)]
pub struct VoiceflowMessage {
    /// The content of the message as a list of `VoiceflowBlock` instances.
    content: Vec<VoiceflowBlock>,
}
impl VoiceflowMessage{
    /// Adds a block to the message content.
    ///
    /// # Parameters
    ///
    /// * `block` - The `VoiceflowBlock` to add to the content.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::voiceflow::{VoiceflowBlock, VoiceflowMessage};
    /// use voiceflousion::core::voiceflow::dialog_blocks::VoiceflowText;
    ///
    /// let mut message = VoiceflowMessage::default();
    /// let block = VoiceflowBlock::Text(VoiceflowText::new("Hello".to_string()));
    /// message.add_block(block);
    /// ```
    pub fn add_block(&mut self, block: VoiceflowBlock) -> (){
        self.content.push(block);
    }

    /// Trims the End block from the message content if it exists.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the end block was trimmed.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::voiceflow::VoiceflowMessage;
    ///
    /// let mut message = VoiceflowMessage::default();
    /// let trimmed = message.trim_end_block();
    /// ```
    pub fn trim_end_block(&mut self) -> bool{
        if let Some(VoiceflowBlock::End) = &self.last() {
            let _ = &self.pop();
            true
        }
        else{
            false
        }
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
/// A builder for creating a `VoiceflowMessage`.
///
/// `VoiceflowMessageBuilder` allows for the incremental construction of a `VoiceflowMessage`
/// by adding blocks and handling the various block types.
pub(crate) struct VoiceflowMessageBuilder;
impl VoiceflowMessageBuilder {
    /// Creates a new `VoiceflowMessageBuilder`.
    ///
    /// # Returns
    ///
    /// A new instance of `VoiceflowMessageBuilder`.
    pub fn new() -> Self {
        Self
    }

    /// Builds a `VoiceflowMessage` from a vector of `VoiceflowResponseBlock` instances.
    ///
    /// # Parameters
    ///
    /// * `blocks` - A vector of `VoiceflowResponseBlock` instances.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `VoiceflowMessage` or a `VoiceflousionError` if the conversion fails.
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
                    VoiceflowResponseBlockType::End => {
                        message.content.push(VoiceflowBlock::End);
                    }
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
                            VoiceflowResponseBlockType::End => {
                                message.content.push(VoiceflowBlock::End);
                            }
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