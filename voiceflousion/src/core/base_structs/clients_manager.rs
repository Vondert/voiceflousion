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
    pub async fn get_client(&self, client_id: &String) -> Option<Arc<C>> {
        let read_lock = self.clients.read().await;
        if let Some(client) = read_lock.get(client_id) {
            return Some(client.clone());
        }
        None
    }

    /// Retrieves all clients.
    ///
    /// # Returns
    ///
    /// A vector of `Arc` containing all clients.
    pub async fn get_all_clients(&self) -> Vec<Arc<C>> {
        let read_lock = self.clients.read().await;
        let clients = read_lock.values().cloned().collect();
        clients
    }

    /// Adds a client to the manager.
    ///
    /// # Parameters
    ///
    /// * `client` - The client to add.
    ///
    /// # Returns
    ///
    /// An `Arc` containing the added client.
    pub async fn add_client(&self, client: C) -> Arc<C> {
        let mut write_lock = self.clients.write().await;
        let client = write_lock.entry(client.client_base().client_id().clone())
            .or_insert_with(|| Arc::new(client))
            .clone();
        client
    }

    /// Deletes a client by its ID.
    ///
    /// # Parameters
    ///
    /// * `client_id` - The ID of the client to delete.
    pub async fn delete_client(&self, client_id: &String) {
        let mut write_lock = self.clients.write().await;
        write_lock.remove(client_id);
    }
}

