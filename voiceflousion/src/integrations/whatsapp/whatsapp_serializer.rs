use serde_json::{json, Value};
use crate::core::voiceflow::dialog_blocks::{VoiceflowButtons, VoiceflowCard};
use crate::core::voiceflow::dialog_blocks::enums::VoiceflowButtonActionType;

/// Serializer for constructing WhatsApp message bodies.
///
/// `WhatsAppSerializer` provides methods to create JSON payloads for different types of WhatsApp messages,
/// including text, image, and interactive (list) messages.
pub(super) struct WhatsAppSerializer {

}

impl WhatsAppSerializer {

    /// Prepares a JSON body for a text message to be sent via WhatsApp.
    ///
    /// # Parameters
    ///
    /// * `chat_id` - The recipient's chat ID.
    /// * `text` - The text content of the message.
    ///
    /// # Returns
    ///
    /// A `Value` containing the structured JSON payload.
    pub fn prepare_text_body(chat_id: &str, text: &str) -> Value {
        json!({
            "messaging_product": "whatsapp",
            "to": chat_id,
            "type": "text",
            "text": {
                "body": text,
            },
        })
    }

    /// Prepares a JSON body for an image message to be sent via WhatsApp.
    ///
    /// # Parameters
    ///
    /// * `chat_id` - The recipient's chat ID.
    /// * `image_url` - The URL of the image to be sent.
    ///
    /// # Returns
    ///
    /// A `Value` containing the structured JSON payload.
    pub fn prepare_image_body(chat_id: &str, image_url: &str) -> Value {
        let mut image_body = json!({
            "messaging_product": "whatsapp",
            "to": chat_id,
            "type": "image",
            "image": {
                "link": image_url,
            }
        });

        image_body
    }

    /// Prepares a JSON body for an interactive list message to be sent via WhatsApp.
    ///
    /// # Parameters
    ///
    /// * `chat_id` - The recipient's chat ID.
    /// * `text` - The text content of the message body.
    /// * `interactive_rows` - A vector of rows representing the list items in the interactive message.
    ///
    /// # Returns
    ///
    /// A `Value` containing the structured JSON payload.
    pub fn prepare_interactive_body(chat_id: &str, text: &str, interactive_rows: Vec<Value>) -> Value {
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

    /// Constructs the parts of a WhatsApp message to be sent, based on the presence of images and interactive rows.
    ///
    /// # Parameters
    ///
    /// * `has_images` - Boolean indicating if the card includes images.
    /// * `card` - A reference to the `VoiceflowCard` object.
    /// * `interactive_rows` - A slice of `Value` representing the interactive rows.
    /// * `chat_id` - The recipient's chat ID.
    ///
    /// # Returns
    ///
    /// A vector of `Value` containing the structured JSON payloads for the card parts.
    pub fn prepare_card_parts(has_images: bool, card: &VoiceflowCard, interactive_rows: &[Value], chat_id: &str) -> Vec<Value> {
        let title = card.title().clone().unwrap_or_default();
        let description = card.description().clone().unwrap_or_default();
        let text = format!("{}\n\n{}", title, description);

        let mut card_parts = Vec::new();

        // If the card has images, prepare the image body part
        if has_images {
            card_parts.push(Self::prepare_image_body(chat_id, card.image_url().as_ref().unwrap()));
        }

        // If interactive rows are provided, prepare the interactive body part
        if !interactive_rows.is_empty() {
            card_parts.push(Self::prepare_interactive_body(chat_id, &text, interactive_rows.to_vec()));
        } else {
            // Otherwise, prepare a simple text body
            card_parts.push(Self::prepare_text_body(chat_id, &text));
        }

        card_parts
    }

    /// Converts a list of `VoiceflowButtons` into a vector of list rows suitable for an interactive WhatsApp message.
    ///
    /// # Parameters
    ///
    /// * `buttons` - A reference to the `VoiceflowButtons` object.
    /// * `buttons_mark` - A mark (i64) associated with the buttons, used in the callback data.
    ///
    /// # Returns
    ///
    /// A vector of `Value` representing the list rows.
    pub fn buttons_to_list_rows(buttons: &VoiceflowButtons, buttons_mark: i64) -> Vec<Value> {
        buttons.iter().enumerate().map(|(index, b)| {
            // Prepare callback data for each button
            let callback_data = json!({
                "index": index,
                "mark": buttons_mark
            });

            let callback_data_string = serde_json::to_string(&callback_data).unwrap_or_else(|_| "".to_string());

            // If the button action type is OpenUrl, use the URL as the description
            let description = if let VoiceflowButtonActionType::OpenUrl(url) =  &b.action_type(){
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

    /// Converts the buttons of a carousel card into a vector of list rows, adding navigation buttons for carousel traversal.
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
    pub fn carousel_card_buttons_to_list_rows(card: &VoiceflowCard, mark: i64, index: usize, carousel_len: usize) -> Vec<Value> {
        let mut list_rows: Vec<Value> = card.buttons().as_ref()
            .map(|b| Self::buttons_to_list_rows(b, mark))
            .unwrap_or_else(|| vec![]);

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