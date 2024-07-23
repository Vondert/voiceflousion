use std::ops::Deref;
use std::time::Duration;
use reqwest::Client;

/// A client for handling HTTP requests with session management.
///
/// `HttpClient` wraps around the `reqwest::Client` to provide additional configurations

pub struct HttpClient {
    /// The HTTP client for making requests.
    client: Client,
    /// The maximum number of idle connections per host.
    max_connections_per_moment: usize,
    /// Duration of the HTTP connection in seconds
    connection_duration: u64
}

impl Deref for HttpClient {
    type Target = Client;

    /// Dereferences to the underlying HTTP client.
    ///
    /// # Returns
    ///
    /// A reference to the `reqwest::Client`.
    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl HttpClient {
    /// Creates a new `HttpClient`.
    ///
    /// # Parameters
    ///
    /// * `max_sessions_per_moment` - The maximum number of idle connections per host.
    /// * `connection_duration` - The optional duration for which connections can remain idle (in seconds).
    ///
    /// # Returns
    ///
    /// A new instance of `HttpClient`.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::subtypes::HttpClient;
    ///
    /// let http_client = HttpClient::new(10, Some(60));
    /// let default_duration_client = HttpClient::new(10, None);
    /// ```
    pub fn new(max_sessions_per_moment: usize, connection_duration: Option<u64>) -> Self {
        let connection_duration = if let Some(duration) = connection_duration{
            duration
        }
        else{
            120
        };
        Self {
            client: Client::builder()
                .pool_max_idle_per_host(max_sessions_per_moment)
                .pool_idle_timeout(Duration::from_secs(connection_duration))
                .build().unwrap(),
            max_connections_per_moment: max_sessions_per_moment,
            connection_duration
        }
    }

    /// Returns the maximum number of idle connections per host.
    ///
    /// # Returns
    ///
    /// A `usize` representing the maximum number of idle connections per host.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::subtypes::HttpClient;
    ///
    /// let http_client = HttpClient::new(10, Some(60));
    ///
    /// let max_connections = http_client.max_connections_per_moment();
    /// ```
    pub fn max_connections_per_moment(&self) -> usize {
        self.max_connections_per_moment
    }

    /// Returns the connection duration.
    ///
    /// # Returns
    ///
    /// A `u64` representing the connection duration in seconds.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::subtypes::HttpClient;
    ///
    /// let http_client = HttpClient::new(10, Some(60));
    ///
    /// let connection_duration = http_client.connection_duration();
    /// ```
    pub fn connection_duration(&self) -> u64{
        self.connection_duration
    }
}
