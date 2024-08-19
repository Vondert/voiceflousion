use crate::core::voiceflow::dialog_blocks::{VoiceflowButtons, VoiceflowCard, VoiceflowCarousel, VoiceflowImage, VoiceflowText};
use crate::core::voiceflow::response_structures::{VoiceflowResponseBlock, VoiceflowResponseBlockType};
use crate::core::voiceflow::{VoiceflowBlock, VoiceflowMessage};
use crate::core::voiceflow::dialog_blocks::enums::VoiceflowButtonsOption;
use crate::core::voiceflow::dialog_blocks::traits::FromValue;

/// Deserializes Voiceflow response blocks and processes them to build a VoiceflowMessage.
pub(crate) struct VoiceflowResponseBlockProcessor;

impl VoiceflowResponseBlockProcessor {
    /// Creates a new instance of VoiceflowResponseBlockProcessor.
    ///
    /// # Returns
    ///
    /// A new instance of VoiceflowResponseBlockProcessor.
    pub fn new() -> Self {
        Self
    }

    /// Processes a Voiceflow response block and updates the message accordingly.
    ///
    /// # Parameters
    ///
    /// * `message` - The message to update.
    /// * `block` - The block to process.
    ///
    /// # Returns
    ///
    /// An updated VoiceflowButtonsOption.
    pub fn process_block(&self, message: &mut VoiceflowMessage, block: VoiceflowResponseBlock) -> VoiceflowButtonsOption {
        match block.block_type() {
            VoiceflowResponseBlockType::Text => {
                self.extract_text(block)
            },
            VoiceflowResponseBlockType::Choice => {
                self.extract_buttons(message, block);
                VoiceflowButtonsOption::Empty
            },
            VoiceflowResponseBlockType::CardV2 => {
                self.extract_card(message, block);
                VoiceflowButtonsOption::Empty
            },
            VoiceflowResponseBlockType::Visual => {
                self.extract_image(message, block);
                VoiceflowButtonsOption::Empty
            },
            VoiceflowResponseBlockType::Carousel => {
                self.extract_carousel(message, block);
                VoiceflowButtonsOption::Empty
            },
            VoiceflowResponseBlockType::End => {
                message.push(VoiceflowBlock::End);
                VoiceflowButtonsOption::Empty
            },
            _ => VoiceflowButtonsOption::Empty,
        }
    }

    /// Processes buttons options and updates the message accordingly.
    ///
    /// # Parameters
    ///
    /// * `message` - The message to update.
    /// * `buttons_options` - The current buttons options.
    /// * `block` - The block to process.
    pub fn process_buttons_options(&self, message: &mut VoiceflowMessage, buttons_options: &mut VoiceflowButtonsOption, block: VoiceflowResponseBlock) {
        match block.block_type() {
            VoiceflowResponseBlockType::Choice => {
                self.add_buttons_with_options(message, buttons_options, block);
            },
            _ => {
                self.add_buttons_options_to_message(message, buttons_options.clone());
                *buttons_options = self.process_block(message, block);
            },
        }
    }

    /// Extracts text from a block.
    ///
    /// # Parameters
    ///
    /// * `block` - The block to extract text from.
    ///
    /// # Returns
    ///
    /// An updated VoiceflowButtonsOption.
    fn extract_text(&self, block: VoiceflowResponseBlock) -> VoiceflowButtonsOption {
        let optional_text = VoiceflowText::from_value(block.json()).unwrap_or_else(|error| {
            println!("{:?}", error);
            Some(VoiceflowText::error_default("Invalid voiceflow text format"))
        });
        if let Some(text) = optional_text {
            VoiceflowButtonsOption::Text(text)
        } else {
            VoiceflowButtonsOption::Empty
        }
    }

    /// Extracts buttons from a block and adds them to the message.
    ///
    /// # Parameters
    ///
    /// * `message` - The message to update.
    /// * `block` - The block to extract buttons from.
    fn extract_buttons(&self, message: &mut VoiceflowMessage, block: VoiceflowResponseBlock) {
        match VoiceflowButtons::from_value(block.json()) {
            Ok(optional_buttons) => {
                if let Some(buttons) = optional_buttons {
                    message.push(VoiceflowBlock::Buttons(buttons));
                }
            }
            Err(error) => {
                println!("{:?}", error);
                self.add_error_text_to_message(message, "Invalid voiceflow buttons format");
            }
        }
    }

    /// Extracts a card from a block and adds it to the message.
    ///
    /// # Parameters
    ///
    /// * `message` - The message to update.
    /// * `block` - The block to extract the card from.
    fn extract_card(&self, message: &mut VoiceflowMessage, block: VoiceflowResponseBlock) {
        match VoiceflowCard::from_value(block.json()) {
            Ok(optional_card) => {
                if let Some(card) = optional_card {
                    message.push(VoiceflowBlock::Card(card));
                }
            }
            Err(error) => {
                println!("{:?}", error);
                self.add_error_text_to_message(message, "Invalid voiceflow card format");
            }
        }
    }

    /// Extracts an image from a block and adds it to the message.
    ///
    /// # Parameters
    ///
    /// * `message` - The message to update.
    /// * `block` - The block to extract the image from.
    fn extract_image(&self, message: &mut VoiceflowMessage, block: VoiceflowResponseBlock) {
        match VoiceflowImage::from_value(block.json()) {
            Ok(optional_image) => {
                if let Some(image) = optional_image {
                    message.push(VoiceflowBlock::Image(image));
                }
            }
            Err(error) => {
                println!("{:?}", error);
                self.add_error_text_to_message(message, "Invalid voiceflow image format");
            }
        }
    }

    /// Extracts a carousel from a block and adds it to the message.
    ///
    /// # Parameters
    ///
    /// * `message` - The message to update.
    /// * `block` - The block to extract the carousel from.
    fn extract_carousel(&self, message: &mut VoiceflowMessage, block: VoiceflowResponseBlock) {
        match VoiceflowCarousel::from_value(block.json()) {
            Ok(optional_carousel) => {
                if let Some(carousel) = optional_carousel {
                    message.push(VoiceflowBlock::Carousel(carousel));
                }
            }
            Err(error) => {
                println!("{:?}", error);
                self.add_error_text_to_message(message, "Invalid voiceflow carousel format");
            }
        }
    }

    /// Adds buttons with options to the message.
    ///
    /// # Parameters
    ///
    /// * `message` - The message to update.
    /// * `buttons_options` - The current buttons options.
    /// * `block` - The block to extract buttons from.
    fn add_buttons_with_options(&self, message: &mut VoiceflowMessage, buttons_options: &mut VoiceflowButtonsOption, block: VoiceflowResponseBlock) {
        match VoiceflowButtons::from_value(block.json()) {
            Ok(optional_buttons) => {
                if let Some(mut buttons) = optional_buttons {
                    buttons.set_option(buttons_options.clone());
                    message.push(VoiceflowBlock::Buttons(buttons));
                    *buttons_options = VoiceflowButtonsOption::Empty;
                }
            }
            Err(error) => {
                println!("{:?}", error);
                self.add_error_text_to_message(message, "Invalid voiceflow buttons format");
            }
        }
    }

    /// Adds an error text block to the message.
    ///
    /// # Parameters
    ///
    /// * `message` - The message to update.
    /// * `error_text` - The error text to add.
    fn add_error_text_to_message(&self, message: &mut VoiceflowMessage, error_text: &str) {
        message.push(VoiceflowBlock::Text(VoiceflowText::error_default(error_text)));
    }

    /// Adds buttons options to the message.
    ///
    /// # Parameters
    ///
    /// * `message` - The message to update.
    /// * `buttons_options` - The buttons options to add.
    pub fn add_buttons_options_to_message(&self, message: &mut VoiceflowMessage, buttons_options: VoiceflowButtonsOption) {
        match buttons_options {
            VoiceflowButtonsOption::Text(text) => message.push(VoiceflowBlock::Text(text)),
            _ => {},
        }
    }
}
