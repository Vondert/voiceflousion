use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::core::traits::Client;

pub struct ClientsManager<C: Client>{
    pub clients: Arc<RwLock<HashMap<String, Arc<C>>>>
}
impl<C: Client> ClientsManager<C>{
    pub fn new() -> Self {
        Self {
            clients: Arc::new(RwLock::new(HashMap::<String, Arc<C>>::new()))
        }
    }

    pub fn from_clients(clients_vec: Vec<C>) -> Self {
        let mut hash_map = HashMap::<String, Arc<C>>::new();
        let _ = clients_vec.into_iter().for_each(|client| {
            hash_map.insert(client.client_base().client_id().clone(), Arc::new(client));
        });
        Self {
            clients: Arc::new(RwLock::new(hash_map))
        }
    }

    pub async fn get_client(&self, client_id: &String) -> Option<Arc<C>> {
        let read_lock = self.clients.read().await;
        if let Some(client) = read_lock.get(client_id) {
            return Some(client.clone());
        }
        None
    }

    pub async fn get_all_clients(&self) -> Vec<Arc<C>> {
        let read_lock = self.clients.read().await;
        let clients = read_lock.values().cloned().collect();
        clients
    }

    pub async fn add_client(&self, client: C) -> Arc<C> {
        let mut write_lock = self.clients.write().await;
        let client = write_lock.entry(client.client_base().client_id().clone())
            .or_insert_with(|| Arc::new(client))
            .clone();
        client
    }

    pub async fn delete_client(&self, client_id: &String) {
        let mut write_lock = self.clients.write().await;
        write_lock.remove(client_id);
    }
}

