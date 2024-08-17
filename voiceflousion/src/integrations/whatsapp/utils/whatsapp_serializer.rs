use serde_json::{json, Value};
use crate::core::voiceflow::dialog_blocks::{VoiceflowButtons, VoiceflowCard};
use crate::core::voiceflow::dialog_blocks::enums::{VoiceflowButtonActionType, VoiceflowButtonsOption};

/// Serializer for constructing WhatsApp message bodies.
///
/// `WhatsAppSerializer` provides methods to create JSON payloads for different types of WhatsApp messages,
/// including text, image, and interactive (buttons and carousel) messages.
pub(crate) struct WhatsAppSerializer {}

impl WhatsAppSerializer {

    /// Builds a JSON body for a text message to be sent via WhatsApp.
    ///
    /// # Parameters
    ///
    /// * `chat_id` - The recipient's chat ID.
    /// * `text` - The text content of the message.
    ///
    /// # Returns
    ///
    /// A `Value` containing the structured JSON payload.
    pub fn build_text_body(chat_id: &str, text: &str) -> Value {
        json!({
            "messaging_product": "whatsapp",
            "to": chat_id,
            "type": "text",
            "text": {
                "body": text,
            },
        })
    }

    /// Builds a JSON body for an image message to be sent via WhatsApp.
    ///
    /// # Parameters
    ///
    /// * `chat_id` - The recipient's chat ID.
    /// * `image_url` - The URL of the image to be sent.
    ///
    /// # Returns
    ///
    /// A `Value` containing the structured JSON payload.
    pub fn build_image_body(chat_id: &str, image_url: &str) -> Value {
        json!({
            "messaging_product": "whatsapp",
            "to": chat_id,
            "type": "image",
            "image": {
                "link": image_url,
            }
        })
    }

    /// Builds a JSON body for an interactive buttons message to be sent via WhatsApp.
    ///
    /// # Parameters
    ///
    /// * `chat_id` - The recipient's chat ID.
    /// * `buttons` - The `VoiceflowButtons` object containing the buttons data.
    ///
    /// # Returns
    ///
    /// A `Value` containing the structured JSON payload.
    pub fn build_buttons_body(chat_id: &str, buttons: &VoiceflowButtons) -> Value {
        let text = match buttons.option() {
            VoiceflowButtonsOption::Text(text) => text.message().clone(),
            VoiceflowButtonsOption::Empty => String::from("Invalid behavior. Please fix errors in WhatsAppSender usage")
        };

        let interactive_rows = Self::build_buttons_vec(buttons, buttons.mark());

        Self::build_buttons_base_body(chat_id, text, interactive_rows)
    }

    /// Builds a JSON body for a carousel card with buttons to be sent via WhatsApp.
    ///
    /// # Parameters
    ///
    /// * `chat_id` - The recipient's chat ID.
    /// * `card` - The `VoiceflowCard` object containing the card data.
    /// * `text` - The text content of the carousel card.
    /// * `mark` - A mark (i64) associated with the buttons, used in the callback data.
    /// * `index` - The current index of the card in the carousel.
    /// * `carousel_len` - The total number of cards in the carousel.
    ///
    /// # Returns
    ///
    /// A `Value` containing the structured JSON payload.
    fn build_carousel_buttons_body(chat_id: &str, card: &VoiceflowCard, text: String, mark: i64, index: usize, carousel_len: usize) -> Value {
        let interactive_rows = Self::build_carousel_card_buttons_vec(card, mark, index, carousel_len);
        Self::build_buttons_base_body(chat_id, text, interactive_rows)
    }

    /// Builds the base structure for a WhatsApp interactive buttons message.
    ///
    /// # Parameters
    ///
    /// * `chat_id` - The recipient's chat ID.
    /// * `text` - The text content of the message.
    /// * `interactive_rows` - A vector of `Value` representing the interactive rows.
    ///
    /// # Returns
    ///
    /// A `Value` containing the structured JSON payload.
    fn build_buttons_base_body(chat_id: &str, text: String, interactive_rows: Vec<Value>) -> Value {
        json!({
            "messaging_product": "whatsapp",
            "to": chat_id,
            "type": "interactive",
            "interactive": {
                "type": "list",
                "body": {
                    "text": text,
                },
                "action": {
                    "button": "👇",
                    "sections": [
                        {
                            "title": "Buttons",
                            "rows": interactive_rows,
                        }
                    ]
                }
            }
        })
    }

    /// Builds the parts of a WhatsApp message to be sent, based on the presence of images and buttons.
    ///
    /// # Parameters
    ///
    /// * `card` - A reference to the `VoiceflowCard` object.
    /// * `chat_id` - The recipient's chat ID.
    ///
    /// # Returns
    ///
    /// A vector of `Value` containing the structured JSON payloads for the card parts.
    pub fn build_card_parts(card: &VoiceflowCard, chat_id: &str) -> Vec<Value> {
        let title = card.title().clone().unwrap_or_default();
        let description = card.description().clone().unwrap_or_default();
        let text = format!("{}\n\n{}", title, description);

        let mut card_parts = Vec::new();

        if let Some(url) = card.image_url() {
            card_parts.push(Self::build_image_body(chat_id, url));
        }

        if let Some(buttons) = card.buttons() {
            card_parts.push(Self::build_buttons_body(chat_id, buttons));
        } else {
            card_parts.push(Self::build_text_body(chat_id, &text));
        }

        card_parts
    }

    /// Builds the parts of a WhatsApp carousel card to be sent, based on the presence of images and buttons.
    ///
    /// # Parameters
    ///
    /// * `card` - A reference to the `VoiceflowCard` object.
    /// * `chat_id` - The recipient's chat ID.
    /// * `mark` - A mark (i64) associated with the buttons, used in the callback data.
    /// * `index` - The current index of the card in the carousel.
    /// * `carousel_len` - The total number of cards in the carousel.
    ///
    /// # Returns
    ///
    /// A vector of `Value` containing the structured JSON payloads for the carousel card parts.
    pub fn build_carousel_card_parts(card: &VoiceflowCard, chat_id: &str, mark: i64, index: usize, carousel_len: usize) -> Vec<Value> {
        let title = card.title().clone().unwrap_or_default();
        let description = card.description().clone().unwrap_or_default();
        let text = format!("{}\n\n{}", title, description);

        let mut card_parts = Vec::new();

        if let Some(url) = card.image_url() {
            card_parts.push(Self::build_image_body(chat_id, url));
        }

        if let Some(_buttons) = card.buttons() {
            card_parts.push(Self::build_carousel_buttons_body(chat_id, card, text, mark, index, carousel_len));
        } else {
            card_parts.push(Self::build_text_body(chat_id, &text));
        }

        card_parts
    }

    /// Builds a list of `VoiceflowButtons` into a vector of list rows suitable for an interactive WhatsApp message.
    ///
    /// # Parameters
    ///
    /// * `buttons` - A reference to the `VoiceflowButtons` object.
    /// * `buttons_mark` - A mark (i64) associated with the buttons, used in the callback data.
    ///
    /// # Returns
    ///
    /// A vector of `Value` representing the list rows.
    fn build_buttons_vec(buttons: &VoiceflowButtons, buttons_mark: i64) -> Vec<Value> {
        buttons.iter().enumerate().map(|(index, b)| {
            let callback_data = json!({
                "index": index,
                "mark": buttons_mark
            });

            let callback_data_string = serde_json::to_string(&callback_data).unwrap_or_else(|_| "".to_string());

            let description = if let VoiceflowButtonActionType::OpenUrl(url) = &b.action_type() {
                url.clone()
            } else {
                String::new()
            };

            json!({
                "id": callback_data_string,
                "title": b.name(),
                "description": description
            })
        }).collect()
    }

    /// Builds the buttons of a carousel card into a vector of list rows, adding navigation buttons for carousel traversal.
    ///
    /// # Parameters
    ///
    /// * `card` - A reference to the `VoiceflowCard` object.
    /// * `mark` - A mark (i64) associated with the buttons, used in the callback data.
    /// * `index` - The current index of the card in the carousel.
    /// * `carousel_len` - The total number of cards in the carousel.
    ///
    /// # Returns
    ///
    /// A vector of `Value` representing the list rows, including navigation buttons.
    fn build_carousel_card_buttons_vec(card: &VoiceflowCard, mark: i64, index: usize, carousel_len: usize) -> Vec<Value> {
        let mut list_rows: Vec<Value> = card.buttons().as_ref()
            .map(|b| Self::build_buttons_vec(b, mark))
            .unwrap_or_else(Vec::new);

        // Add a previous button if this is not the first card
        if index > 0 {
            let carousel_prev = json!({
                "direction": format!("{}", false),
                "mark": mark
            });
            list_rows.push(json!({
                "id": carousel_prev.to_string(),
                "title": "<--",
                "description": ""
            }));
        }

        // Add a next button if this is not the last card
        if index < carousel_len - 1 {
            let carousel_next = json!({
                "direction": format!("{}", true),
                "mark": mark
            });
            list_rows.push(json!({
                "id": carousel_next.to_string(),
                "title": "-->",
                "description": ""
            }));
        }

        list_rows
    }
}