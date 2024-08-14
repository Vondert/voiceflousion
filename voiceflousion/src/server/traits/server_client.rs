use axum::http::StatusCode;
use axum::Json;
use axum_core::response::{IntoResponse, Response};
use serde_json::Value;
use crate::core::subtypes::BotAuthToken;
use crate::core::traits::Client;
use crate::integrations::telegram::TelegramClient;
use crate::integrations::whatsapp::WhatsAppClient;
use crate::server::subtypes::QueryParams;

pub trait ServerClient: Client{
    #[inline]
    fn authenticate_webhook(_params: &mut QueryParams, _value: Option<&Value>, _bot_auth_token: Option<BotAuthToken>) -> Option<Response>{
        None
    }
}

impl ServerClient for WhatsAppClient{
    fn authenticate_webhook(params: &mut QueryParams, value: Option<&Value>, bot_auth_token: Option<BotAuthToken>) -> Option<Response>{
        if let Some(json) = value{
            let origin_type = json["entry"][0]["changes"][0]["value"]["statuses"][0]["conversation"]["origin"]["type"]
                .as_str();

            if origin_type == Some("service") {
                return Some((StatusCode::OK, Json("Service type Update rejected".to_string())).into_response());
            }

            return None;
        }
        if let Some(challenge) = params.remove("hub.challenge") {
            if let Some(bot_token) = bot_auth_token{
                if let Some(verify_token) = params.remove("hub.verify_token"){
                    if bot_token.token() != &verify_token{
                        return Some((StatusCode::OK, Json("Webhook authorization failed!".to_string())).into_response());
                    }
                }
                else{
                    return Some((StatusCode::OK, Json("Webhook authorization failed!".to_string())).into_response());
                }
            }
            return Some((StatusCode::OK, challenge).into_response());
        }
        None
    }
}

#[cfg(feature = "telegram")]
impl ServerClient for TelegramClient{}