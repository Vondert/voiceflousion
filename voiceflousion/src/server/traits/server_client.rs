use axum_core::response::Response;
use serde_json::Value;
use crate::core::subtypes::BotAuthToken;
use crate::core::traits::Client;
use crate::integrations::telegram::TelegramClient;
use crate::integrations::whatsapp::WhatsAppClient;
use crate::server::subtypes::QueryParams;

pub trait ServerClient: Client{
    fn authenticate_webhook(_params: &QueryParams, _value: Option<Value>, _bot_auth_token: Option<BotAuthToken>) -> Option<Response>{
        println!("Generic");
        None
    }
}

impl ServerClient for WhatsAppClient{
    fn authenticate_webhook(params: &QueryParams, value: Option<Value>, bot_auth_token: Option<BotAuthToken>) -> Option<Response>{
        println!("WhatsApp");
        None
    }
}

impl ServerClient for TelegramClient{}