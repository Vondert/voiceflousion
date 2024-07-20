use std::sync::Arc;
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
}