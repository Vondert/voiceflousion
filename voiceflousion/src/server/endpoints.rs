use std::collections::HashMap;
use std::sync::Arc;
use axum::extract::{Path, Query};
use axum::{Extension, Json};
use axum::http::StatusCode;
use axum_core::response::IntoResponse;
use serde_json::Value;
use crate::core::base_structs::ClientsManager;
use crate::core::traits::Update;
use crate::server::subtypes::{AuthResult, QueryParams, VoiceflousionHeadersWrapper};
use crate::server::traits::{BotHandler, ServerClient};

/// Main endpoint function for handling incoming webhook requests.
///
/// This function processes the incoming request, validates the client and authentication token,
/// checks the origin header, and deserializes the update before passing it to the handler function.
///
/// # Parameters
///
/// * `id` - The ID of the bot client extracted from the request path.
/// * `params` - Query parameters containing additional request data and the bot authentication token.
/// * `body` - The JSON body of the incoming request.
/// * `headers` - Wrapper for the HTTP headers in the request.
/// * `clients` - The clients manager containing the bot clients.
/// * `optional_allowed_origins` - Optional allowed origins for CORS settings.
/// * `handler` - The handler function for processing the update.
///
/// # Returns
///
/// A response indicating the result of the request processing.
pub(super) async fn main_endpoint<C: ServerClient>(
    id: Path<String>,
    mut params: Query<QueryParams>,
    Json(body): Json<Value>,
    headers: VoiceflousionHeadersWrapper,
    Extension(clients): Extension<Arc<ClientsManager<C>>>,
    Extension(optional_allowed_origins): Extension<Arc<Option<HashMap<&'static str, ()>>>>,
    Extension(handler): Extension<Arc<dyn BotHandler<C>>>
) -> impl IntoResponse {
    // Authenticate the request, including checking the origin and token.
    let client = match authenticate_request::<C>(id, &mut params, Some(&body), headers, clients.clone(), optional_allowed_origins.clone()).await{
        AuthResult::Client(client) => client,
        AuthResult::Response(response) => return response
    };

    // Deserialize the update from the request body.
    let update = match deserialize_update::<C::ClientUpdate<'static>>(body) {
        Ok(update) => update,
        Err(err) => {
            println!("Error deserializing update: {:?}", err);
            return (StatusCode::OK, Json("Invalid update".to_string())).into_response();
        }
    };

    // Check if the client is active
    if !client.client_base().is_active() {
        println!("Client {} deactivated!", client.client_base().client_id());
        return (StatusCode::OK, Json("Access to deactivated client".to_string())).into_response();
    }

    // Process the update using the handler function
    match handler(update, client).await {
        Ok(_) => (StatusCode::OK, Json("Ok".to_string())).into_response(),
        Err(_) => (StatusCode::OK, Json("Handler error".to_string())).into_response(),
    }
}

/// Authentication endpoint for GET requests.
///
/// This function handles authentication requests for GET endpoints by validating
/// the client ID, checking the origin header, and verifying the authentication token.
///
/// # Parameters
///
/// * `id` - The ID of the bot client extracted from the request path.
/// * `params` - Query parameters containing additional request data and the bot authentication token.
/// * `headers` - Wrapper for the HTTP headers in the request.
/// * `clients` - The clients manager containing the bot clients.
/// * `optional_allowed_origins` - Optional allowed origins for CORS settings.
///
/// # Returns
///
/// A response indicating the result of the authentication.
pub(super) async fn get_auth_endpoint<C: ServerClient>(
    id: Path<String>,
    mut params: Query<QueryParams>,
    headers: VoiceflousionHeadersWrapper,
    Extension(clients): Extension<Arc<ClientsManager<C>>>,
    Extension(optional_allowed_origins): Extension< Arc<Option<HashMap<&'static str, ()>>>>
) -> impl IntoResponse{
    match authenticate_request::<C>(id, &mut params, None, headers, clients.clone(), optional_allowed_origins.clone()).await{
        AuthResult::Client(_client) => {
            (StatusCode::OK, Json("Endpoint authentication with GET response not passed".to_string())).into_response()
        },
        AuthResult::Response(response) => response
    }
}

/// Deserializes the incoming JSON body into the appropriate update type.
///
/// This function attempts to deserialize the incoming JSON body into the type expected
/// by the client's update handling logic.
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

/// Authenticates an incoming request.
///
/// This function handles client ID validation, token authentication, and origin header checks.
/// If the request is authenticated successfully, it returns the client object; otherwise, it returns a response indicating the failure.
///
/// # Parameters
///
/// * `id` - The ID of the bot client extracted from the request path.
/// * `params` - The query parameters extracted from the request, including the bot authentication token.
/// * `body` - The optional JSON body of the request.
/// * `headers` - Wrapper for the HTTP headers in the request.
/// * `clients` - The clients manager containing the bot clients.
/// * `optional_allowed_origins` - Optional allowed origins for CORS settings.
///
/// # Returns
///
/// An `AuthResult` indicating either the authenticated client or a response indicating the failure.
async fn authenticate_request<C: ServerClient>(
    Path(id): Path<String>,
    Query(params): &mut Query<QueryParams>,
    body: Option<&Value>,
    headers: VoiceflousionHeadersWrapper,
    clients: Arc<ClientsManager<C>>,
    optional_allowed_origins: Arc<Option<HashMap<&'static str, ()>>>
) -> AuthResult<C> {
    // Check the Origin header if allowed origins are defined.
    if let Some(allowed_origins) = &*optional_allowed_origins {
        if let Some(origin) = headers.get_header_str("Origin"){
            if !allowed_origins.contains_key(origin) {
                println!("Unauthorized origin: {}", origin);
                return AuthResult::Response((StatusCode::OK, Json("Unauthorized origin".to_string())).into_response());
            }
        }
        else{
            println!("Missing origin header in request! Client: {}\nAdd headers to request or turn of allowed origins in server!", id);
            return AuthResult::Response((StatusCode::OK, Json("Missing origin header in request! Add headers to request or turn of allowed origins in server!".to_string())).into_response());
        }
    }

    // Validate client ID
    let client = if let Some(client) = clients.get_client(&id).await {
        client
    } else {
        println!("Invalid client id - {}", id);
        return AuthResult::Response((StatusCode::OK, Json("Invalid client id".to_string())).into_response());
    };

    // Validate authentication token if present
    if let Some(client_token) = client.client_base().bot_auth_token().await {
        let bot_auth_token = if let Some(token) = params.extract_bot_auth_token() {
            token
        } else {
            println!("Missing token parameter");
            return AuthResult::Response((StatusCode::OK, Json("Missing token parameter".to_string())).into_response());
        };

        // Check if the token matches
        if client_token.token() != bot_auth_token.token() {
            println!("Unauthorized access to client {}", client.client_base().client_id());
            return AuthResult::Response((StatusCode::OK, Json("Unauthorized access".to_string())).into_response());
        }

        // Perform server client request authentication if required
        if let Some(response) = client.authenticate_server_client_request(headers, params, body, Some(bot_auth_token)){
            return AuthResult::Response(response);
        };
    }
    else{
        // Perform authentication without a token
        if let Some(response) = client.authenticate_server_client_request(headers, params, body, None){
            return AuthResult::Response(response);
        };
    };

    // Return the authenticated client
    AuthResult::Client(client)
}