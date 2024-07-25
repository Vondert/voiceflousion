use crate::core::subtypes::HttpClient;

/// `SenderBase` is the foundational struct for managing HTTP requests and interactions with the API.
///
/// This struct encapsulates essential components such as the HTTP client for sending requests
/// and the API key for authenticating with the API.
pub struct SenderBase{
    /// The HTTP client for sending requests.
    http_client: HttpClient,
    /// The API key for authenticating with the API.
    api_key: String,
}
impl SenderBase{

    /// Creates a new `SenderBase`.
    ///
    /// # Parameters
    ///
    /// * `max_sessions_per_moment` - The maximum number of idle connections per host.
    /// * `api_key` - The API key for authenticating with the API.
    /// * `connection_duration` - The optional duration for which sessions can remain idle (in seconds).
    ///
    /// # Returns
    ///
    /// A new instance of `SenderBase`.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::base_structs::SenderBase;
    ///
    /// let sender = SenderBase::new(10, "api_key".to_string(), Some(120));
    /// ```
    pub fn new(max_sessions_per_moment: usize, api_key: String, connection_duration: Option<u64>) -> Self {
        Self {
            http_client: HttpClient::new(max_sessions_per_moment, connection_duration),
            api_key,
        }
    }

    /// Returns a reference to the HTTP client used for sending requests.
    ///
    /// # Returns
    ///
    /// A reference to the `HttpClient`.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::base_structs::SenderBase;
    ///
    /// let sender = SenderBase::new(10, "api_key".to_string(), Some(120));
    /// let http_client = sender.http_client();
    /// ```
    pub fn http_client(&self) -> &HttpClient{
        &self.http_client
    }

    /// Returns a reference to the API key used for authentication.
    ///
    /// # Returns
    ///
    /// A reference to the API key string.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::base_structs::SenderBase;
    ///
    /// let sender = SenderBase::new(10, "api_key".to_string(), Some(120));
    /// let api_key = sender.api_key();
    /// ```
    pub fn api_key(&self) -> &String{
        &self.api_key
    }
}