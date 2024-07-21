use std::sync::Arc;
use crate::integrations::utils::ClientBuilder;
use crate::integrations::utils::sessions_manager::SessionsManager;
use crate::integrations::utils::traits::{Sender, Update};
use crate::voiceflow::VoiceflowClient;

/// A trait that defines the base functionalities for a client.
///
/// The `ClientBase` trait provides basic methods to access client ID, session manager,
/// Voiceflow client, and the message sender. It serves as a foundation for client
/// implementations that need to interact with Voiceflow and manage sessions.
pub trait ClientBase: Sync + Send {
    type ClientUpdate: Update;
    type ClientSender: Sender;
    /// Returns a reference to the client ID.
    ///
    /// # Returns
    ///
    /// A reference to the client ID string.
    fn client_id(&self) -> &String;

    /// Returns a reference to the session manager.
    ///
    /// # Returns
    ///
    /// A reference to the session manager wrapped in an `Arc`.
    fn sessions(&self) -> &Arc<SessionsManager>;

    /// Returns a reference to the Voiceflow client.
    ///
    /// # Returns
    ///
    /// A reference to the Voiceflow client wrapped in an `Arc`.
    fn voiceflow_client(&self) -> &Arc<VoiceflowClient>;

    /// Returns a reference to the message sender.
    ///
    /// # Returns
    ///
    /// A reference to the message sender.
    fn sender(&self) -> &Self::ClientSender;

    /// Destructures the client into a `ClientBuilder` without sessions.
    ///
    /// This method creates a `ClientBuilder` with the client's current configurations,
    /// excluding the sessions.
    ///
    /// # Returns
    ///
    /// A `ClientBuilder` instance without sessions.
    ///
    /// # Example
    ///
    /// ```
    /// let builder = client.destructure_to_client_builder_without_sessions();
    /// ```
    fn destructure_to_client_builder_without_sessions(&self) -> ClientBuilder{
        let client_id = self.client_id().clone();
        let api_key = self.sender().api_key().clone();
        let voiceflow_client = self.voiceflow_client().clone();
        let max_connections_per_moment = self.sender().http_client().max_connections_per_moment();

        let mut builder = ClientBuilder::new(client_id, api_key, voiceflow_client, max_connections_per_moment);
        builder = if let Some(interval) =  self.sessions().cleanup_interval(){
            builder.allow_sessions_cleaning(interval)
        }
        else {
            builder
        };
        if let Some(duration) = self.sessions().valid_session_duration(){
            builder.add_session_duration(duration)
        }
        else {
            builder
        }
    }
}