use std::net::SocketAddr;
use std::sync::Arc;
use axum::{Extension, Router};
use axum::routing::post;
use crate::core::base_structs::ClientsManager;
use crate::core::traits::{Client};
use crate::server::endpoints::{authenticate_webhook_endpoint, main_endpoint};
use crate::server::traits::{BotHandler, ServerClient};

/// VoiceflousionServer is responsible for handling HTTP requests to bots and routing them to the appropriate handlers.
///
/// This struct contains bots clients manager, base part of the server's HTTP endpoint URL, and the handler function for processing incoming webhook requests.
pub struct VoiceflousionServer<C: ServerClient + 'static> {
    /// Manager for handling multiple bots clients.
    clients: Option<Arc<ClientsManager<C >>>,
    /// Base part of the server's HTTP endpoint URL.
    base_url: String,
    /// Handler function for processing incoming webhook requests.
    handler: Arc<dyn BotHandler<C>>,
    /// Allowed origins for CORS settings.
    allowed_origins: Arc<Option<Vec<&'static str>>>,
}

impl<C: ServerClient + 'static> VoiceflousionServer<C> {
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
            handler,
            allowed_origins: Arc::new(None)
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


    /// Enables default allowed origins settings using predefined origins.
    ///
    /// # Returns
    ///
    /// The updated `VoiceflousionServer` instance with default allowed origins settings applied.
    ///
    /// If no predefined origins are set, it allows any origin.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::server::VoiceflousionServer;
    /// use voiceflousion::integrations::telegram::TelegramClient;
    /// use voiceflousion::server::handlers::base_dialog_handler;
    ///
    /// let voiceflousion_telegram_server = VoiceflousionServer::<TelegramClient>::new("telegram".to_string(), {
    ///             |update, client| Box::pin(base_dialog_handler(update, client))
    /// })
    /// .enable_default_origins();
    /// ```
    pub fn enable_default_origins(mut self) -> Self {
        let origins = C::ORIGINS.to_vec();

        self.allowed_origins = Arc::new(if origins.is_empty() {
            None
        } else {
            Some(origins)
        });
        self
    }

    /// Overrides the allowed origins for server settings.
    ///
    /// # Parameters
    ///
    /// * `origins` - A vector of strings containing the origins to allow.
    ///
    /// # Returns
    ///
    /// The updated `VoiceflousionServer` instance with overridden allowed origins settings.
    ///
    /// This method extends the provided origins with the predefined origins.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::server::VoiceflousionServer;
    /// use voiceflousion::integrations::telegram::TelegramClient;
    /// use voiceflousion::server::handlers::base_dialog_handler;
    ///
    /// let additional_origins = vec![
    ///     "http://example.com",
    ///     "http://another-example.com",
    /// ];
    ///
    /// let voiceflousion_telegram_server = VoiceflousionServer::<TelegramClient>::new("telegram".to_string(), {
    ///             |update, client| Box::pin(base_dialog_handler(update, client))
    /// })
    /// .override_allow_origins(additional_origins);
    /// ```
    pub fn override_allow_origins(mut self, mut origins: Vec<&'static str>) -> Self {
        origins.extend(C::ORIGINS);
        self.allowed_origins = Arc::new(if origins.is_empty() {
            None
        } else {
            Some(origins)
        });
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
    ///         .run(([127, 0, 0, 1], 8080))
    ///         .await
    ///     });
    /// }
    /// ```
    pub async fn run(self, address: impl Into<SocketAddr>) {
        let base_url = self.base_url.clone();
        let router = self.create_router(&base_url).await.into_make_service();

        // Start the HTTP server
        let ip = address.into();
        let listener = tokio::net::TcpListener::bind(ip).await.unwrap();

        println!("Server is set on {}/{}", ip, base_url);
        println!("Bots without authentication token are available on {}/{}/<bot_id>", ip, base_url);
        println!("Bots with authentication token are available on {}/{}/<bot_id>/?token=<token>", ip, base_url);

        axum::serve(listener, router).await.unwrap();
    }


    /// Creates a new router with the specified base URL.
    ///
    /// # Parameters
    ///
    /// * `base_url` - The base part of the server's HTTP endpoint URL.
    ///
    /// # Returns
    ///
    /// A `Router` instance configured with the appropriate routes and CORS settings.
    async fn create_router(self, base_url: &String) -> Router{
        let clients = self.clients.clone().expect("Webhook is not set");
        let handler = self.handler.clone();
        let optional_allowed_origins = self.allowed_origins.clone();

        Router::new()
            .route(&format!("/{}/:id", base_url),
                   post({
                       let clients = clients.clone();
                       let optional_allowed_origins = optional_allowed_origins.clone();
                       let handler = handler.clone();
                       move |origin_header, path, params, json| {
                           main_endpoint(
                               path,
                               params,
                               json,
                               Extension(clients),
                               Extension(optional_allowed_origins),
                               Extension(handler),
                               origin_header,
                           )
                       }
                   })
                   .get({
                       let clients = clients.clone();
                       let optional_allowed_origins = optional_allowed_origins.clone();
                       move |origin_header, path, params| {
                            authenticate_webhook_endpoint(
                                path,
                                params,
                                Extension(clients),
                                Extension(optional_allowed_origins),
                                origin_header
                            )
                       }
                   }),
            )
    }
}