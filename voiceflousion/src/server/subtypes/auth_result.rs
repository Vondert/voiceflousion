use std::sync::Arc;
use axum_core::response::Response;
use crate::server::traits::ServerClient;

/// Enum representing the result of an authentication process.
///
/// This enum is used to handle the outcome of an authentication attempt. It can either
/// return the authenticated client or an HTTP response indicating an error or status.
///
/// # Variants
///
/// * `Client` - Represents a successful authentication, containing the authenticated client.
/// * `Response` - Represents a failed authentication or other status, containing the HTTP response.
pub(crate) enum AuthResult<C: ServerClient> {
    /// Variant holding the authenticated client.
    Client(Arc<C>),

    /// Variant holding the HTTP response to be returned.
    Response(Response),
}