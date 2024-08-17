use axum::http::StatusCode;
use axum::Json;
use axum_core::response::{IntoResponse, Response};
use serde_json::Value;
use crate::core::subtypes::BotAuthToken;
use crate::core::traits::Client;
use crate::integrations::telegram::TelegramClient;
use crate::integrations::whatsapp::WhatsAppClient;
use crate::server::subtypes::QueryParams;

/// Trait that extends the `Client` trait to add server-specific functionality.
///
/// `ServerClient` is designed to be implemented by clients that interact with the server
/// and require additional logic for webhook authentication. This trait provides a default
/// method for authenticating webhooks, which can be overridden by specific client implementations.
pub trait ServerClient: Client {
    /// Authenticates incoming webhook requests.
    ///
    /// This method is used to validate webhook requests by examining the query parameters,
    /// the optional JSON body, and an optional bot authentication token. By default, it
    /// returns `None`, meaning no authentication is performed.
    ///
    /// # Parameters
    ///
    /// * `_params` - The query parameters extracted from the request.
    /// * `_value` - The optional JSON body of the request.
    /// * `_bot_auth_token` - The optional bot authentication token.
    ///
    /// # Returns
    ///
    /// An optional `Response` indicating the result of the authentication. If `None` is returned,
    /// the request is considered authenticated.
    #[inline]
    fn authenticate_webhook(
        _params: &mut QueryParams,
        _value: Option<&Value>,
        _bot_auth_token: Option<BotAuthToken>
    ) -> Option<Response> {
        None
    }
}

/// Implementation of `ServerClient` for `WhatsAppClient`.
///
/// This implementation overrides the `authenticate_webhook` method to provide specific
/// authentication logic for WhatsApp webhook requests.
#[cfg(feature = "whatsapp")]
impl ServerClient for WhatsAppClient{
    fn authenticate_webhook(params: &mut QueryParams, value: Option<&Value>, bot_auth_token: Option<BotAuthToken>) -> Option<Response>{
        // Check if the incoming webhook update is of type "service" and reject it
        if let Some(json) = value{
            let origin_type = json["entry"][0]["changes"][0]["value"]["statuses"][0]["conversation"]["origin"]["type"]
                .as_str();

            if origin_type == Some("service") {
                return Some((StatusCode::OK, Json("Service type Update rejected".to_string())).into_response());
            }

            return None;
        }

        // Handle the webhook verification challenge for WhatsApp
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

/// Implementation of `ServerClient` for `TelegramClient`.
///
/// Since Telegram does not require additional authentication logic beyond the default,
/// this implementation uses the default `authenticate_webhook` method provided by the `ServerClient` trait.
#[cfg(feature = "telegram")]
impl ServerClient for TelegramClient{}