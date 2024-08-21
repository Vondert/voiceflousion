use axum::http::StatusCode;
use axum::Json;
use axum_core::response::{IntoResponse, Response};
use serde_json::{json, Value};
use crate::core::subtypes::BotAuthToken;
use crate::core::traits::Client;
use crate::server::subtypes::{QueryParams, VoiceflousionHeadersWrapper};

#[cfg(feature = "telegram")]
use crate::integrations::telegram::TelegramClient;
#[cfg(feature = "whatsapp")]
use crate::integrations::whatsapp::WhatsAppClient;
#[cfg(feature = "instagram")]
use crate::integrations::instagram::InstagramClient;
#[cfg(feature = "discord_unimplemented")]
use crate::{
    integrations::discord::DiscordClient,
    server::traits::utils::discord_public_key_verify
};

/// Trait that extends the `Client` trait to add server-specific functionality.
///
/// `ServerClient` is designed to be implemented by clients that interact with the server
/// and require additional logic for server client requests authentication. This trait provides a default
/// method for authenticating server client requests, which can be overridden by specific client implementations.
pub trait ServerClient: Client {

    /// A list of allowed origins for CORS.
    ///
    /// This constant defines an array of static string slices representing the origins
    /// that are allowed to make cross-origin requests to the client's server. These origins are
    /// used to configure the CORS settings of the server, ensuring that only requests
    /// from specified origins are permitted.
    const ORIGINS: &'static [&'static str];

    /// The base URL path for the server client.
    ///
    /// This constant defines the base URL path that is used by the client to handle incoming requests.
    const BASE_URL: &'static str;

    /// Authenticates incoming requests to the server client.
    ///
    /// This method is used to validate server client requests by examining the HTTP headers,
    /// query parameters, the optional JSON body, and an optional bot authentication token.
    /// By default, it returns `None`, meaning no authentication is performed.
    ///
    /// # Parameters
    ///
    /// * `_headers` - The HTTP headers extracted from the request.
    /// * `_params` - The query parameters extracted from the request.
    /// * `_value` - The optional JSON body of the request.
    /// * `_bot_auth_token` - The optional bot authentication token.
    ///
    /// # Returns
    ///
    /// An optional `Response` indicating the result of the authentication. If `None` is returned,
    /// the request is considered authenticated.
    #[inline]
    fn authenticate_server_client_request(
        &self,
        _headers: VoiceflousionHeadersWrapper,
        _params: &mut QueryParams,
        _value: Option<&Value>,
        _bot_auth_token: Option<BotAuthToken>
    ) -> Option<Response> {
        None
    }
}

/// Implementation of `ServerClient` for `WhatsAppClient`.
///
/// This implementation overrides the `authenticate_server_client_request` method to provide specific
/// authentication logic for WhatsApp server client requests.
#[cfg(feature = "whatsapp")]
impl ServerClient for WhatsAppClient {
    /// Allowed origins for CORS specific to the WhatsApp client.
    const ORIGINS: &'static [&'static str] = &[];

    /// Base URL path for the WhatsApp client.
    const BASE_URL: &'static str = "whatsapp";

    fn authenticate_server_client_request(
        &self,
        _headers: VoiceflousionHeadersWrapper,
        params: &mut QueryParams,
        value: Option<&Value>,
        bot_auth_token: Option<BotAuthToken>
    ) -> Option<Response> {
        // Check if the incoming request is of type "service" and reject it
        if let Some(json) = value {
            let origin_type = json["entry"][0]["changes"][0]["value"]["statuses"][0]["conversation"]["origin"]["type"]
                .as_str();

            if origin_type == Some("service") {
                return Some((StatusCode::OK, Json("Service type Update rejected".to_string())).into_response());
            }

            return None;
        }

        // Handle the request verification for WhatsApp
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
/// this implementation uses the default `authenticate_server_client_request` method provided by the `ServerClient` trait.
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

    /// Base URL path for the Telegram client.
    const BASE_URL: &'static str = "telegram";
}

#[cfg(feature = "instagram")]
impl ServerClient for InstagramClient{
    const ORIGINS: &'static [&'static str] = &[];
    const BASE_URL: &'static str = "instagram";
}

/// Implementation of `ServerClient` for `DiscordClient`.
///
/// This implementation overrides the `authenticate_server_client_request` method to provide specific
/// authentication logic for Discord server client requests, including signature verification.
#[cfg(feature = "discord_unimplemented")]
impl ServerClient for DiscordClient {
    /// Allowed origins for CORS specific to the Discord client.
    const ORIGINS: &'static [&'static str] = &[];

    /// Base URL path for the Discord client.
    const BASE_URL: &'static str = "discord";

    fn authenticate_server_client_request(
        &self,
        headers: VoiceflousionHeadersWrapper,
        _params: &mut QueryParams,
        value: Option<&Value>,
        _bot_auth_token: Option<BotAuthToken>
    ) -> Option<Response> {
        // Check if the request body exists and extract it
        let body = match value {
            Some(body) => body,
            None => return Some((StatusCode::UNAUTHORIZED, Json("Invalid request body".to_string())).into_response())
        };

        // Extract signature and timestamp headers
        let signature = headers.get_header_str_or_empty("x-signature-ed25519");
        let timestamp = headers.get_header_str_or_empty("x-signature-timestamp");

        // Return an error response if either header is missing
        if timestamp.is_empty() || signature.is_empty() {
            return Some((StatusCode::UNAUTHORIZED, Json("Timestamp and signature headers aren't provided".to_string())).into_response());
        }

        // Verify the request signature using the Discord public key
        let public_key = self.get_public_key();
        if let Err(error) = discord_public_key_verify(public_key, signature, timestamp, body) {
            println!("Discord authentication error: {}", error);
            return Some((StatusCode::UNAUTHORIZED, Json("Key verification failed".to_string())).into_response());
        }

        // Handle different types of Discord requests
        let request_type = body.get("type").and_then(|type_value| type_value.as_u64()).unwrap_or_default();

        match request_type {
            1 => {
                let body = Json(json!({
                    "type": request_type
                }));

                Some((StatusCode::OK, body).into_response())
            },
            4 => None,
            _ => Some((StatusCode::UNAUTHORIZED, Json("Invalid request type".to_string())).into_response())
        }
    }
}
