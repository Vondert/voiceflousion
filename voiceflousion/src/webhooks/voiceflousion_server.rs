use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use axum::http::StatusCode;
use axum::{Json, Router, };
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::post;
use axum_core::response::Response;
use futures::future::BoxFuture;
use serde_json::Value;
use crate::core::base_structs::ClientsManager;
use crate::core::traits::{Client, Update};
use crate::errors::VoiceflousionResult;

pub type BotHandler<U, C> = Arc<dyn Fn(U, Arc<C>) -> BoxFuture<'static, VoiceflousionResult<()>> + Send + Sync>;

pub struct VoiceflousionServer<C: Client + 'static> {
    clients: Option<Arc<ClientsManager<C>>>,
    base_url: Option<String>,
    handler: Arc<BotHandler<C::ClientUpdate<'static>, C>>
}

impl<C: Client + 'static> VoiceflousionServer<C> {
    pub fn new(webhook_handler: Arc<BotHandler<C::ClientUpdate<'static>, C>>) -> Self {
        Self{
            clients: None,
            base_url: None,
            handler: webhook_handler
        }
    }
    pub fn set_webhook(mut self, base_url: String, clients: Arc<ClientsManager<C>>) -> Self{
        self.clients = Some(clients);
        self.base_url = Some(base_url);
        self
    }
    pub async fn serve(self, address: impl Into<SocketAddr>) {
        let clients = self.clients.clone().expect("Webhook is not set");
        let handler = self.handler.clone();
        let base_url = self.base_url.clone().expect("Webhook is not set");

        let app = Router::new()
            .route(
                &format!("/{}/:id", base_url),
                post(move |Path(id): Path<String>, Json(body): Json<Value>| {
                    let clients = clients.clone();
                    let handler = handler.clone();
                    async move {
                        if let Some(client) = clients.get_client(&id).await {
                            let update = match deserialize_update::<C::ClientUpdate<'static>>(body) {
                                Ok(update) => update,
                                Err(err) => {
                                    println!("Error deserializing update: {:?}", err);
                                    return Ok::<Response, Infallible>(
                                        Json("Ok".to_string()).into_response(),
                                    );
                                }
                            };
                            match handler(update, client).await {
                                Ok(_) => Ok::<Response, Infallible>(
                                    Json("Ok".to_string()).into_response(),
                                ),
                                Err(_) => Ok::<Response, Infallible>(
                                    Json("Ok".to_string()).into_response(),
                                ),
                            }
                        } else {
                            Ok::<Response, Infallible>(Json("Ok".to_string()).into_response())
                        }
                    }
                }),
            );
        let ip = address.into();
        let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
        println!("Server is set on {}/{}", ip, base_url);
        println!("Bots are available on {}/{}/<bot_id>", ip, base_url);
        axum::serve(listener, app).await.unwrap();
    }
}

fn deserialize_update<U: Update>(body: Value) -> Result<U, StatusCode> {
    match U::from_request_body(body) {
        Ok(update) => Ok(update),
        Err(err) => {
            println!("Error: {:?}", &err);
            Err(StatusCode::OK)
        }
    }
}