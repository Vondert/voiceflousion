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
use crate::server::BotHandler;

/// VoiceflousionServer is responsible for handling HTTP requests to bots and routing them to the appropriate handlers.
///
/// This struct contains bots clients manager, base part of the server's HTTP endpoint URL, and the handler function for processing incoming webhook requests.
pub struct VoiceflousionServer<C: Client + 'static> {
    /// Manager for handling multiple bots clients.
    clients: Option<Arc<ClientsManager<C>>>,
    /// Base part of the server's HTTP endpoint URL.
    base_url: String,
    /// Handler function for processing incoming webhook requests.
    handler: Arc<dyn BotHandler<C>>
}

impl<C: Client + 'static> VoiceflousionServer<C> {
    /// Creates a new instance of `VoiceflousionServer`.
    ///
    /// # Parameters
    ///
    /// * `base_url` - Base part of the server's HTTP endpoint URL.
    /// * `webhook_handler` - The handler  function for processing incoming webhook requests.
    ///
    /// # Returns
    ///
    /// A new instance of `VoiceflousionServer`.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::integrations::telegram::TelegramClient;
    /// use voiceflousion::server::handlers::base_dialog_handler;
    /// use voiceflousion::server::VoiceflousionServer;
    ///
    /// let voiceflousion_telegram_server = VoiceflousionServer::<TelegramClient>::new("telegram".to_string(), {
    ///             |update, client| Box::pin(base_dialog_handler(update, client))
    ///  });
    /// ```
    pub fn new(base_url: String, webhook_handler: impl BotHandler<C> + 'static) -> Self {
        let handler =  Arc::new(move |update: C::ClientUpdate<'static>, client: Arc<C>| {
            webhook_handler(update, client)
        });
        Self {
            clients: None,
            base_url,
            handler
        }
    }

    /// Sets the bots clients manager.
    ///
    /// # Parameters
    ///
    /// * `clients` - An `Arc` containing the bots clients manager.
    ///
    /// # Returns
    ///
    /// The updated `VoiceflousionServer` instance.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::base_structs::ClientsManager;
    /// use voiceflousion::core::ClientBuilder;
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    /// use voiceflousion::integrations::telegram::TelegramClient;
    /// use voiceflousion::server::handlers::base_dialog_handler;
    /// use voiceflousion::server::VoiceflousionServer;
    ///
    /// let voiceflow_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, Some(120)));
    /// let builder = ClientBuilder::new("client_id".to_string(), "api_key".to_string(), voiceflow_client, 10);
    /// let telegram_client = TelegramClient::new(builder);
    ///
    /// let telegram_client_manager = Arc::new(ClientsManager::from_clients(vec![telegram_client]));
    ///
    /// let voiceflousion_telegram_server = VoiceflousionServer::<TelegramClient>::new("telegram".to_string(), {
    ///             |update, client| Box::pin(base_dialog_handler(update, client))
    ///  })
    ///  .set_clients_manager(telegram_client_manager);
    /// ```
    pub fn set_clients_manager(mut self, clients: Arc<ClientsManager<C>>) -> Self {
        self.clients = Some(clients);
        self
    }

    /// Starts the server and begins listening for incoming requests.
    ///
    /// # Parameters
    ///
    /// * `address` - The address to bind the server to.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::base_structs::ClientsManager;
    /// use voiceflousion::core::ClientBuilder;
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    /// use voiceflousion::integrations::telegram::TelegramClient;
    /// use voiceflousion::server::handlers::base_dialog_handler;
    /// use voiceflousion::server::VoiceflousionServer;
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///     let voiceflow_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, Some(120)));
    ///     let builder = ClientBuilder::new("client_id".to_string(), "api_key".to_string(), voiceflow_client, 10);
    ///     let telegram_client = TelegramClient::new(builder);
    ///
    ///     let telegram_client_manager = Arc::new(ClientsManager::from_clients(vec![telegram_client]));
    ///
    ///     tokio::spawn(async move {
    ///         VoiceflousionServer::<TelegramClient>::new("telegram".to_string(), {
    ///             |update, client| Box::pin(base_dialog_handler(update, client))
    ///         })
    ///         .set_clients_manager(telegram_client_manager)
    ///         .serve(([127, 0, 0, 1], 8080))
    ///         .await
    ///     });
    /// }
    /// ```
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
                            // Validate client ID
                            let client = if let Some(client) = clients.get_client(&id).await {
                                client
                            } else {
                                println!("Invalid client id - {}", id);
                                return Ok::<Response, Infallible>(Json("Invalid client id".to_string()).into_response());
                            };

                            // Validate authentication token if present
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

                            // Check if the client is active
                            if !client.client_base().is_active() {
                                println!("Client {} deactivated!", id);
                                return Ok::<Response, Infallible>(Json("Access to deactivated client".to_string()).into_response());
                            }

                            // Deserialize the update
                            let update = match deserialize_update::<C::ClientUpdate<'static>>(body) {
                                Ok(update) => update,
                                Err(err) => {
                                    println!("Error deserializing update: {:?}", err);
                                    return Ok::<Response, Infallible>(Json("Invalid update".to_string()).into_response());
                                }
                            };

                            // Process the update using the handler function
                            match handler(update, client).await {
                                Ok(_) => Ok::<Response, Infallible>(Json("Ok".to_string()).into_response()),
                                Err(_) => Ok::<Response, Infallible>(Json("Handler error".to_string()).into_response())
                            }
                        }
                    }
                }),
            );

        // Start the HTTP server
        let ip = address.into();
        let listener = tokio::net::TcpListener::bind(ip).await.unwrap();

        println!("Server is set on {}/{}", ip, base_url);
        println!("Bots without authentication token are available on {}/{}/<bot_id>", ip, base_url);
        println!("Bots with authentication token are available on {}/{}/<bot_id>/?token=<token>", ip, base_url);

        axum::serve(listener, app).await.unwrap();
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