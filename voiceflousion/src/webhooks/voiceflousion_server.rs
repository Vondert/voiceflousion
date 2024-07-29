use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use axum::http::StatusCode;
use axum::{Json, Router};
use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::routing::post;
use axum_core::response::Response;
use serde_json::Value;
use crate::core::base_structs::ClientsManager;
use crate::core::subtypes::BotAuthToken;
use crate::core::traits::{Client, Update};
use crate::webhooks::handlers::BotHandler;

pub struct VoiceflousionServer<C: Client + 'static> {
    clients: Option<Arc<ClientsManager<C>>>,
    base_url: String,
    handler: Arc<dyn BotHandler<C>>
}

impl<C: Client + 'static> VoiceflousionServer<C> {
    pub fn new(base_url: String, webhook_handler: Arc<dyn BotHandler<C>>) -> Self {
        Self{
            clients: None,
            base_url,
            handler: webhook_handler
        }
    }
    pub fn set_clients_manager(mut self, clients: Arc<ClientsManager<C>>) -> Self{
        self.clients = Some(clients);
        self
    }
    pub async fn serve(self, address: impl Into<SocketAddr>) {
        let clients = self.clients.clone().expect("Webhook is not set");
        let handler = self.handler.clone();
        let base_url = self.base_url.clone();

        let app = Router::new()
            .route(
                &format!("/{}/:id", base_url),
                post({
                    let clients = clients.clone();
                    let handler = handler.clone();
                    move |Path(id): Path<String>, params: Option<Query<BotAuthToken>>, Json(body): Json<Value>| {
                        async move {
                            let client = if let Some(client) = clients.get_client(&id).await {
                                client
                            } else {
                                println!("Invalid client id - {}", id);
                                return Ok::<Response, Infallible>(Json("Invalid client id".to_string()).into_response());
                            };

                            if let Some(client_token) = client.client_base().bot_auth_token().await {
                                let params = if let Some(params) = params {
                                    params
                                } else {
                                    println!("Missing token parameter");
                                    return Ok::<Response, Infallible>(Json("Unauthorized access".to_string()).into_response());
                                };
                                if client_token.token() != params.token() {
                                    println!("Unauthorized access to client {}", client.client_base().client_id());
                                    return Ok::<Response, Infallible>(Json("Unauthorized access".to_string()).into_response());
                                }
                            };

                            if !client.client_base().is_active() {
                                println!("Client {} deactivated!", id);
                                return Ok::<Response, Infallible>(Json("Access to deactivated client".to_string()).into_response());
                            }

                            let update = match deserialize_update::<C::ClientUpdate<'static>>(body) {
                                Ok(update) => update,
                                Err(err) => {
                                    println!("Error deserializing update: {:?}", err);
                                    return Ok::<Response, Infallible>(Json("Invalid update".to_string()).into_response());
                                }
                            };

                            match handler(update, client).await {
                                Ok(_) => Ok::<Response, Infallible>(Json("Ok".to_string()).into_response()),
                                Err(_) => Ok::<Response, Infallible>(Json("Handler error".to_string()).into_response())
                            }
                        }
                    }
                }),
            );
        let ip = address.into();
        let listener = tokio::net::TcpListener::bind(ip).await.unwrap();

        println!("Server is set on {}/{}", ip, base_url);
        println!("Bots without authentication token are available on {}/{}/<bot_id>", ip, base_url);
        println!("Bots with authentication token are available on {}/{}/<bot_id>/?token=<token>", ip, base_url);

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