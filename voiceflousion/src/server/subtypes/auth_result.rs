use std::sync::Arc;
use axum_core::response::Response;
use crate::server::traits::ServerClient;

pub(crate) enum AuthResult<C: ServerClient>{
    Client(Arc<C>),
    Response(Response)
}