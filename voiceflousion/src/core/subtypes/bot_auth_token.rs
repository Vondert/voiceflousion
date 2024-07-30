use serde::Deserialize;

/// Represents a bot authentication token.
///
/// `BotAuthToken` is a structure that holds the token string used for authenticating
/// the bot in Voiceflousion server. It provides methods for creating a new token
/// and accessing the token string.
#[derive(Debug, Clone, Deserialize)]
pub struct BotAuthToken {
    /// The authentication token string.
    token: String,
}

impl BotAuthToken {
    /// Creates a new `BotAuthToken`.
    ///
    /// # Parameters
    ///
    /// * `token` - The authentication token string.
    ///
    /// # Returns
    ///
    /// A new instance of `BotAuthToken`.
    pub(crate) fn new(token: String) -> Self {
        Self { token }
    }

    /// Returns a reference to the token string.
    ///
    /// # Returns
    ///
    /// A reference to the token string.
    pub fn token(&self) -> &String {
        &self.token
    }
}