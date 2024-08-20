use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use serde::Deserialize;
use crate::core::subtypes::BotAuthToken;

/// A struct representing query parameters in a request.
///
/// `QueryParams` wraps a `HashMap<String, String>` where each key-value pair represents
/// a parameter and its associated value. This struct also provides utility methods
/// for extracting specific parameters, such as the bot authentication token.
#[derive(Deserialize, Debug)]
pub struct QueryParams {
    #[serde(flatten)]
    params: HashMap<String, String>,
}

impl Deref for QueryParams {
    type Target = HashMap<String, String>;

    /// Deref implementation to allow direct access to the underlying `HashMap`.
    ///
    /// This enables the use of `QueryParams` as if it were a `HashMap` directly.
    fn deref(&self) -> &Self::Target {
        &self.params
    }
}

impl DerefMut for QueryParams {
    /// DerefMut implementation to allow mutable access to the underlying `HashMap`.
    ///
    /// This enables mutable operations on `QueryParams` as if it were a `HashMap` directly.
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.params
    }
}

impl QueryParams {
    /// Extracts the bot authentication token from the query parameters.
    ///
    /// This method searches for a key named "voiceflousion_bot_token" in the query parameters,
    /// removes it from the `HashMap`, and returns it as a `BotAuthToken` if found.
    ///
    /// # Returns
    ///
    /// An `Option<BotAuthToken>` containing the token if found, or `None` if the key does not exist.
    pub(crate) fn extract_bot_auth_token(&mut self) -> Option<BotAuthToken> {
        if let Some(token) = self.remove("voiceflousion_bot_token") {
            Some(BotAuthToken::new(token))
        } else {
            None
        }
    }
}
