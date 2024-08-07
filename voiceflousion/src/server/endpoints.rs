use std::sync::Arc;
use axum::extract::{Path, Query};
use axum::{Extension, Json};
use axum::http::StatusCode;
use axum_extra::TypedHeader;
use axum_core::response::IntoResponse;
use axum_extra::headers::Origin;
use serde_json::Value;
use crate::core::base_structs::ClientsManager;
use crate::core::subtypes::BotAuthToken;
use crate::core::traits::{Client, Update};
use crate::server::BotHandler;

/// Main endpoint function for handling incoming webhook requests.
///
/// This function processes the incoming request, validates the client and authentication token,
/// checks the origin header, and deserializes the update before passing it to the handler function.
///
/// # Parameters
///
/// * `id` - The ID of the bot client.
/// * `params` - Optional query parameters containing the bot authentication token.
/// * `body` - The JSON body of the incoming request.
/// * `clients` - The clients manager containing the bot clients.
/// * `optional_allowed_origins` - Optional allowed origins for allowed origins settings.
/// * `handler` - The handler function for processing the update.
/// * `optional_origin_header` - Optional origin header from the request.
///
/// # Returns
///
/// A response indicating the result of the request processing.
pub(super) async fn main_endpoint_function<C: Client>(
    Path(id): Path<String>,
    params: Option<Query<BotAuthToken>>,
    Json(body): Json<Value>,
    Extension(clients): Extension<Arc<ClientsManager<C>>>,
    Extension(optional_allowed_origins): Extension<Arc<Option<Vec<&str>>>>,
    Extension(handler): Extension<Arc<dyn BotHandler<C>>>,
    optional_origin_header: Option<TypedHeader<Origin>>
) -> impl IntoResponse {

    // Check the Origin header
    if let Some(allowed_origins) = &*optional_allowed_origins {
        if let Some(origin_header) = optional_origin_header{
            let origin = origin_header.hostname();
            if !allowed_origins.iter().any(|allowed_origin| *allowed_origin == origin) {
                println!("Unauthorized origin: {}", origin);
                return (StatusCode::OK, Json("Unauthorized origin".to_string())).into_response();
            }
        }
        else{
            println!("Missing origin header in request! Client: {}\nAdd headers to request or turn of allowed origins in server!", id);
            return (StatusCode::OK, Json("Missing origin header in request! Add headers to request or turn of allowed origins in server!".to_string())).into_response();
        }

    }

    // Validate client ID
    let client = if let Some(client) = clients.get_client(&id).await {
        client
    } else {
        println!("Invalid client id - {}", id);
        return (StatusCode::OK, Json("Invalid client id".to_string())).into_response();
    };

    // Validate authentication token if present
    if let Some(client_token) = client.client_base().bot_auth_token().await {
        let params = if let Some(params) = params {
            params
        } else {
            println!("Missing token parameter");
            return (StatusCode::OK, Json("Unauthorized access".to_string())).into_response();
        };
        if client_token.token() != params.token() {
            println!("Unauthorized access to client {}", client.client_base().client_id());
            return (StatusCode::OK, Json("Unauthorized access".to_string())).into_response();
        }
    };

    // Check if the client is active
    if !client.client_base().is_active() {
        println!("Client {} deactivated!", id);
        return (StatusCode::OK, Json("Access to deactivated client".to_string())).into_response();
    }

    // Deserialize the update
    let update = match deserialize_update::<C::ClientUpdate<'static>>(body) {
        Ok(update) => update,
        Err(err) => {
            println!("Error deserializing update: {:?}", err);
            return (StatusCode::OK, Json("Invalid update".to_string())).into_response();
        }
    };

    // Process the update using the handler function
    match handler(update, client).await {
        Ok(_) => (StatusCode::OK, Json("Ok".to_string())).into_response(),
        Err(_) => (StatusCode::OK, Json("Handler error".to_string())).into_response(),
    }
}

/// Deserializes the incoming JSON body into the appropriate update type.
///
/// # Parameters
///
/// * `body` - The JSON body to deserialize.
///
/// # Returns
///
/// A `Result` containing either the deserialized update or a `StatusCode` indicating an error.
fn deserialize_update<U: Update>(body: Value) -> Result<U, StatusCode> {
    match U::from_request_body(body) {
        Ok(update) => Ok(update),
        Err(err) => {
            println!("Error: {:?}", &err);
            Err(StatusCode::OK)
        }
    }
}