use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::core::traits::Client;

/// Manages multiple bots clients.
///
/// `ClientsManager` provides functionalities to manage and interact with multiple bots clients.
/// It allows adding, retrieving, and deleting clients, as well as fetching all clients at once.
pub struct ClientsManager<C: Client> {
    /// A thread-safe map of client IDs to clients.
    pub clients: Arc<RwLock<HashMap<String, Arc<C>>>>
}

impl<C: Client> ClientsManager<C>{
    /// Creates a new `ClientsManager`.
    ///
    /// # Returns
    ///
    /// A new instance of `ClientsManager`.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::base_structs::ClientsManager;
    /// use voiceflousion::integrations::telegram::TelegramClient;
    ///
    /// let clients_manager: ClientsManager<TelegramClient> = ClientsManager::new();
    /// ```
    pub fn new() -> Self {
        Self {
            clients: Arc::new(RwLock::new(HashMap::<String, Arc<C>>::new()))
        }
    }

    /// Creates a `ClientsManager` from a vector of clients.
    ///
    /// # Parameters
    ///
    /// * `clients_vec` - A vector of clients to initialize the manager with.
    ///
    /// # Returns
    ///
    /// A new instance of `ClientsManager` initialized with the provided clients.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use voiceflousion::core::base_structs::ClientsManager;
    /// use voiceflousion::integrations::telegram::TelegramClient;
    ///
    /// let clients_vector: Vec<TelegramClient> = vec![];
    /// let clients_manager: ClientsManager<TelegramClient> = ClientsManager::from_clients(clients_vector);
    /// ```
    pub fn from_clients(clients_vec: Vec<C>) -> Self {
        let mut hash_map = HashMap::<String, Arc<C>>::new();
        let _ = clients_vec.into_iter().for_each(|client| {
            hash_map.insert(client.client_base().client_id().clone(), Arc::new(client));
        });
        Self {
            clients: Arc::new(RwLock::new(hash_map))
        }
    }

    /// Retrieves a client by its ID.
    ///
    /// # Parameters
    ///
    /// * `client_id` - The ID of the client to retrieve.
    ///
    /// # Returns
    ///
    /// An `Option` containing an `Arc` to the client if found, or `None` if not.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    /// use voiceflousion::core::base_structs::ClientsManager;
    /// use voiceflousion::integrations::telegram::TelegramClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let clients_manager: ClientsManager<TelegramClient> = ClientsManager::new();
    ///     let client_id = String::from("client_123");
    ///     let client = clients_manager.get_client(&client_id).await;
    /// }
    /// ```
    pub async fn get_client(&self, client_id: &String) -> Option<Arc<C>> {
        let read_lock = self.clients.read().await;
        read_lock.get(client_id).cloned()
    }

    /// Retrieves all clients.
    ///
    /// # Returns
    ///
    /// A vector of `Arc` containing all clients.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    /// use voiceflousion::core::base_structs::ClientsManager;
    /// use voiceflousion::integrations::telegram::TelegramClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let clients_manager: ClientsManager<TelegramClient> = ClientsManager::new();
    ///     let clients = clients_manager.get_all_clients().await;
    ///     println!("Number of clients: {}", clients.len());
    /// }
    /// ```
    pub async fn get_all_clients(&self) -> Vec<Arc<C>> {
        let read_lock = self.clients.read().await;
        read_lock.values().cloned().collect()
    }

    /// Adds a client to the manager.
    ///
    /// If a client with the same ID already exists, it will not be replaced.
    ///
    /// # Parameters
    ///
    /// * `client` - The client to add.
    ///
    /// # Returns
    ///
    /// An `Arc` containing the added client.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    /// use voiceflousion::core::base_structs::ClientsManager;
    /// use voiceflousion::integrations::telegram::TelegramClient;
    /// use voiceflousion::core::ClientBuilder;
    /// use voiceflousion::core::voiceflow::VoiceflowClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let clients_manager: ClientsManager<TelegramClient> = ClientsManager::new();
    ///     let voiceflow_client = Arc::new(VoiceflowClient::new("vf_api_key".to_string(), "bot_id".to_string(), "version_id".to_string(), 10, Some(120)));
    ///     let builder = ClientBuilder::new("client_id".to_string(), "api_key".to_string(), voiceflow_client, 10);
    ///     let client = TelegramClient::new(builder);
    ///     clients_manager.add_client(client).await;
    /// }
    /// ```
    pub async fn add_client(&self, client: C) -> Arc<C> {
        let mut write_lock = self.clients.write().await;
        write_lock.entry(client.client_base().client_id().clone())
            .or_insert_with(|| Arc::new(client))
            .clone()
    }

    /// Deletes a client by its ID.
    ///
    /// If the client ID does not exist, no action is taken.
    ///
    /// # Parameters
    ///
    /// * `client_id` - The ID of the client to delete.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    /// use voiceflousion::core::base_structs::ClientsManager;
    /// use voiceflousion::integrations::telegram::TelegramClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let clients_manager: ClientsManager<TelegramClient> = ClientsManager::new();
    ///     let client_id = String::from("client_123");
    ///     clients_manager.delete_client(&client_id).await;
    /// }
    /// ```
    pub async fn delete_client(&self, client_id: &String) {
        let mut write_lock = self.clients.write().await;
        write_lock.remove(client_id);
    }
}

