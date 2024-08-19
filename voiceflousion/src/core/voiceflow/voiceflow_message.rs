use std::fmt::Debug;
use std::vec::IntoIter;
use crate::core::voiceflow::dialog_blocks::enums::VoiceflowButtonsOption;
use crate::core::voiceflow::response_structures::{VoiceflowResponseBlock, VoiceflowResponseBlockProcessor};
use crate::core::voiceflow::VoiceflowBlock;

/// Represents a message from a Voiceflow response.
///
/// `VoiceflowMessage` contains a list of `VoiceflowBlock` instances that make up the content of the message.
#[derive(Debug)]
pub struct VoiceflowMessage {
    /// The content of the message as a list of `VoiceflowBlock` instances.
    content: Vec<VoiceflowBlock>,
}

impl VoiceflowMessage {
    /// Adds a block to the message content.
    ///
    /// # Parameters
    ///
    /// * `block` - The `VoiceflowBlock` to add to the content.
    pub fn add_block(&mut self, block: VoiceflowBlock) {
        self.content.push(block);
    }

    /// Trims the `End` block from the message content if it exists.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the `End` block was trimmed.
    pub fn trim_end_block(&mut self) -> bool {
        if let Some(VoiceflowBlock::End) = &self.content.last() {
            self.content.pop();
            true
        } else {
            false
        }
    }

    /// Inserts a block at the beginning of the message content.
    ///
    /// # Parameters
    ///
    /// * `block` - The `VoiceflowBlock` to insert.
    pub fn shift_block(&mut self, block: VoiceflowBlock) {
        self.content.insert(0, block);
    }

    /// Appends a block to the end of the message content.
    ///
    /// # Parameters
    ///
    /// * `block` - The `VoiceflowBlock` to append.
    pub fn push(&mut self, block: VoiceflowBlock) {
        self.content.push(block);
    }

    /// Returns the number of blocks in the message content.
    ///
    /// # Returns
    ///
    /// The number of blocks in the message as a `usize`.
    pub fn len(&self) -> usize {
        self.content.len()
    }
}

impl Default for VoiceflowMessage {
    /// Creates an empty `VoiceflowMessage`.
    fn default() -> Self {
        Self {
            content: Vec::new(),
        }
    }
}

impl IntoIterator for VoiceflowMessage {
    type Item = VoiceflowBlock;
    type IntoIter = IntoIter<VoiceflowBlock>;

    /// Converts the `VoiceflowMessage` into an iterator over its blocks.
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
    /// This method processes each response block, converts it to a `VoiceflowBlock`, and adds it to the message.
    ///
    /// # Parameters
    ///
    /// * `blocks` - A vector of `VoiceflowResponseBlock` instances.
    ///
    /// # Returns
    ///
    /// A `VoiceflowMessage` containing the processed blocks.
    pub fn build_message(self, blocks: Vec<VoiceflowResponseBlock>) -> VoiceflowMessage {
        let block_processor = VoiceflowResponseBlockProcessor::new();

        let mut message = VoiceflowMessage {
            content: Vec::with_capacity(blocks.len()),
        };
        let mut buttons_options = VoiceflowButtonsOption::Empty;

        for block in blocks {
            if let VoiceflowButtonsOption::Empty = buttons_options {
                buttons_options = block_processor.process_block(&mut message, block);
            } else {
                block_processor.process_buttons_options(&mut message, &mut buttons_options, block);
            }
        }
        block_processor.add_buttons_options_to_message(&mut message, buttons_options);
        message
    }
}