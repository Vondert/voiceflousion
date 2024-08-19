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

    /// A list of allowed origins for CORS.
    ///
    /// This constant defines an array of static string slices representing the origins
    /// that are allowed to make cross-origin requests to the client's server. These origins are
    /// used to configure the CORS settings of the server, ensuring that only requests
    /// from specified origins are permitted.
    const ORIGINS: &'static [&'static str];

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
impl ServerClient for WhatsAppClient {
    /// Allowed origins for CORS specific to the WhatsApp client.
    const ORIGINS: &'static [&'static str] = &[];

    fn authenticate_webhook(params: &mut QueryParams, value: Option<&Value>, bot_auth_token: Option<BotAuthToken>) -> Option<Response> {
        // Check if the incoming webhook update is of type "service" and reject it
        if let Some(json) = value {
            let origin_type = json["entry"][0]["changes"][0]["value"]["statuses"][0]["conversation"]["origin"]["type"]
                .as_str();

            if origin_type == Some("service") {
                return Some((StatusCode::OK, Json("Service type Update rejected".to_string())).into_response());
            }

            return None;
        }

        // Handle the webhook verification challenge for WhatsApp
        if let Some(challenge) = params.remove("hub.challenge") {
            if let Some(bot_token) = bot_auth_token {
                if let Some(verify_token) = params.remove("hub.verify_token") {
                    if bot_token.token() != &verify_token {
                        return Some((StatusCode::OK, Json("Webhook authorization failed!".to_string())).into_response());
                    }
                } else {
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
impl ServerClient for TelegramClient {
    /// An array of allowed origins for CORS specific to the Telegram client.
    const ORIGINS: &'static [&'static str] = &[
        "http://149.154.160.0",
        "http://149.154.160.1",
        "http://149.154.160.2",
        "http://149.154.160.3",
        "http://149.154.160.4",
        "http://149.154.160.5",
        "http://149.154.160.6",
        "http://149.154.160.7",
        "http://149.154.160.8",
        "http://149.154.160.9",
        "http://149.154.160.10",
        "http://149.154.160.11",
        "http://149.154.160.12",
        "http://149.154.160.13",
        "http://149.154.160.14",
        "http://149.154.160.15",
        "http://149.154.160.16",
        "http://149.154.160.17",
        "http://149.154.160.18",
        "http://149.154.160.19",
        "http://149.154.160.20",

        "http://91.108.4.0",
        "http://91.108.4.1",
        "http://91.108.4.2",
        "http://91.108.4.3",
        "http://91.108.4.4",
        "http://91.108.4.5",
        "http://91.108.4.6",
        "http://91.108.4.7",
        "http://91.108.4.8",
        "http://91.108.4.9",
        "http://91.108.4.10",
        "http://91.108.4.11",
        "http://91.108.4.12",
        "http://91.108.4.13",
        "http://91.108.4.14",
        "http://91.108.4.15",
        "http://91.108.4.16",
        "http://91.108.4.17",
        "http://91.108.4.18",
        "http://91.108.4.19",
        "http://91.108.4.20",
        "http://91.108.4.21",
        "http://91.108.4.22"
    ];
}