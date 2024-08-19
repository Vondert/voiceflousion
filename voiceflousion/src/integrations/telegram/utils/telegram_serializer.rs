use serde_json::{json, Value};
use crate::core::voiceflow::dialog_blocks::enums::VoiceflowButtonsOption;
use crate::core::voiceflow::dialog_blocks::{VoiceflowButtons, VoiceflowCard};
use crate::integrations::utils::ButtonCallbackDataBuilder;

pub(crate) struct TelegramSerializer;

impl TelegramSerializer {
    /// Builds the JSON body for sending a text message via Telegram API.
    ///
    /// # Parameters
    ///
    /// * `chat_id` - The chat ID of the recipient.
    /// * `text` - The text message to send.
    ///
    /// # Returns
    ///
    /// A `Value` containing the JSON body for the request.
    pub fn build_text_body(chat_id: &str, text: &str) -> Value {
        json!({
            "chat_id": chat_id,
            "text": text,
        })
    }

    /// Builds the JSON body for sending an image message via Telegram API.
    ///
    /// # Parameters
    ///
    /// * `chat_id` - The chat ID of the recipient.
    /// * `image_url` - The URL of the image to send.
    ///
    /// # Returns
    ///
    /// A `Value` containing the JSON body for the request.
    pub fn build_image_body(chat_id: &str, image_url: &str) -> Value {
        json!({
            "chat_id": chat_id,
            "photo": image_url,
        })
    }

    /// Builds the JSON body for sending a message with buttons via Telegram API.
    ///
    /// # Parameters
    ///
    /// * `chat_id` - The chat ID of the recipient.
    /// * `buttons` - The `VoiceflowButtons` to send.
    ///
    /// # Returns
    ///
    /// A `Value` containing the JSON body for the request.
    pub fn build_buttons_body(chat_id: &str, buttons: &VoiceflowButtons) -> Value {
        let text = match buttons.option() {
            VoiceflowButtonsOption::Text(text) => text.message().clone(),
            VoiceflowButtonsOption::Empty => String::from("Invalid behavior. Please fix errors in TelegramSender usage")
        };

        let inline_keyboard: Vec<Vec<Value>> = Self::build_buttons_vec(buttons);

        json!({
            "chat_id": chat_id,
            "text": text,
            "reply_markup": {
                "inline_keyboard": inline_keyboard,
            }
        })
    }

    /// Builds the JSON body for sending a card message via Telegram API.
    ///
    /// # Parameters
    ///
    /// * `chat_id` - The chat ID of the recipient.
    /// * `card` - The `VoiceflowCard` to send.
    ///
    /// # Returns
    ///
    /// A `Value` containing the JSON body for the request.
    pub fn build_card_body(chat_id: &str, card: &VoiceflowCard) -> Value {
        let title = card.title().clone().unwrap_or(String::new());
        let description = card.description().clone().unwrap_or(String::new());

        let text = format!("{}\n\n{}", title, description);

        let inline_keyboard: Vec<Vec<Value>> = card.buttons().as_ref()
            .map(|b| Self::build_buttons_vec(b))
            .unwrap_or_else(Vec::new);

        Self::build_card_base_body(chat_id, text, card.image_url(), inline_keyboard)
    }

    /// Builds the JSON body for sending a carousel card message via Telegram API.
    ///
    /// # Parameters
    ///
    /// * `chat_id` - The chat ID of the recipient.
    /// * `card` - The `VoiceflowCard` to send.
    /// * `index` - The current position of the card within the carousel (0-based index).
    /// * `carousel_length` - The total number of cards in the carousel.
    ///
    /// # Returns
    ///
    /// A `Value` containing the JSON body for the request.
    pub fn build_carousel_card_body(chat_id: &str, card: &VoiceflowCard, index: usize, carousel_length: usize) -> Value {
        let title = card.title().clone().unwrap_or(String::new());
        let description = card.description().clone().unwrap_or(String::new());

        let text = format!("{}\n\n{}", title, description);

        let inline_keyboard: Vec<Vec<Value>> = Self::build_carousel_card_buttons_vec(card, index, carousel_length);

        Self::build_card_base_body(chat_id, text, card.image_url(), inline_keyboard)
    }

    /// Builds the JSON body for updating a carousel card message via Telegram API.
    ///
    /// # Parameters
    ///
    /// * `chat_id` - The chat ID of the recipient.
    /// * `message_id` - The ID of the message to update.
    /// * `card` - The `VoiceflowCard` to update in the carousel.
    /// * `has_image` - A boolean indicating if the card has an image.
    /// * `index` - The current position of the card within the carousel (0-based index).
    /// * `carousel_length` - The total number of cards in the carousel.
    ///
    /// # Returns
    ///
    /// A `Value` containing the JSON body for the request.
    pub fn build_carousel_update_card_body(chat_id: &str, message_id: &str, card: &VoiceflowCard, has_image: bool, index: usize, carousel_length: usize) -> Value {
        let title = card.title().clone().unwrap_or(String::new());
        let description = card.description().clone().unwrap_or(String::new());

        let text = format!("{}\n\n{}", title, description);

        let inline_keyboard: Vec<Vec<Value>> = Self::build_carousel_card_buttons_vec(card, index, carousel_length);

        if has_image {
            json!({
                "chat_id": chat_id,
                "message_id": message_id,
                "media": {
                    "type": "photo",
                    "media": card.image_url().as_ref().unwrap().clone(),
                    "caption": text,
                },
                "reply_markup": {
                    "inline_keyboard": inline_keyboard,
                }
            })
        } else {
            json!({
                "chat_id": chat_id,
                "message_id": message_id,
                "text": text,
                "reply_markup": {
                    "inline_keyboard": inline_keyboard,
                }
            })
        }
    }

    /// Builds the base JSON body for sending a card or carousel message via Telegram API.
    ///
    /// # Parameters
    ///
    /// * `chat_id` - The chat ID of the recipient.
    /// * `text` - The text content of the message.
    /// * `image_url` - An optional URL of the image to send with the card.
    /// * `inline_keyboard` - The inline keyboard buttons to include in the message.
    ///
    /// # Returns
    ///
    /// A `Value` containing the JSON body for the request.
    fn build_card_base_body(chat_id: &str, text: String, image_url: &Option<String>, inline_keyboard: Vec<Vec<Value>>) -> Value {
        match image_url {
            None => {
                json!({
                    "chat_id": chat_id,
                    "text": text,
                    "reply_markup": {
                        "inline_keyboard": inline_keyboard,
                    }
                })
            }
            Some(url) => {
                json!({
                    "chat_id": chat_id,
                    "photo": url,
                    "caption": text,
                    "reply_markup": {
                        "inline_keyboard": inline_keyboard,
                    }
                })
            }
        }
    }

    /// Converts `VoiceflowButtons` to a keyboard layout for Telegram inline keyboard.
    ///
    /// # Parameters
    ///
    /// * `buttons` - The `VoiceflowButtons` to convert.
    ///
    /// # Returns
    ///
    /// A vector of vectors containing the keyboard layout in JSON format.
    fn build_buttons_vec(buttons: &VoiceflowButtons) -> Vec<Vec<Value>> {
        buttons.iter().enumerate().map(|(index, b)| {
            let callback_data =ButtonCallbackDataBuilder::new().index(index).build().to_json_string();

            json!({ "text": b.name(), "callback_data": callback_data })
        }).map(|key| vec![key]).collect()
    }

    /// Converts the buttons of a `VoiceflowCard` into a Telegram-compatible inline keyboard,
    /// adding navigation buttons for carousel movement.
    ///
    /// This function generates an inline keyboard for a given `VoiceflowCard`, converting its
    /// buttons into a format suitable for use in Telegram's API. Additionally, it appends
    /// navigation buttons ("<--" and "-->") to allow users to move through a carousel of cards.
    /// The navigation buttons are conditionally added based on the card's position in the carousel
    /// and the total number of cards.
    ///
    /// # Parameters
    ///
    /// * `card` - A reference to the `VoiceflowCard` whose buttons will be converted.
    /// * `index` - The current position of the card within the carousel (0-based index).
    /// * `carousel_len` - The total number of cards in the carousel.
    ///
    /// # Returns
    ///
    /// A `Vec<Vec<Value>>` representing the inline keyboard structure for Telegram,
    /// including both the card's buttons and any applicable navigation buttons.
    fn build_carousel_card_buttons_vec(card: &VoiceflowCard, index: usize, carousel_len: usize) -> Vec<Vec<Value>> {
        let mut inline_keyboard: Vec<Vec<Value>> = card.buttons().as_ref()
            .map(Self::build_buttons_vec)
            .unwrap_or_else(Vec::new);

        let mut switch_buttons: Vec<Value> = Vec::new();
        // Add a previous button if this is not the first card
        if index > 0 {
            let carousel_prev= ButtonCallbackDataBuilder::new().direction(false).build().to_json_string();
            switch_buttons.push(json!({ "text": "<--", "callback_data": carousel_prev}));
        }
        // Add a next button if this is not the last card
        if index < carousel_len - 1 {
            let carousel_next= ButtonCallbackDataBuilder::new().direction(true).build().to_json_string();
            switch_buttons.push(json!({ "text": "-->", "callback_data": carousel_next }));
        }
        inline_keyboard.push(switch_buttons);

        inline_keyboard
    }
}