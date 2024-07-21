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
    /// Creates a new `SenderHttpClient`.
    ///
    /// # Parameters
    ///
    /// * `max_connections_per_moment` - The maximum number of idle connections per host.
    ///
    /// # Returns
    ///
    /// A new instance of `SenderHttpClient`.
    ///
    /// # Example
    ///
    /// ```
    /// let http_client = SenderHttpClient::new(10);
    /// ```
    pub fn new(max_sessions_per_moment: usize) -> Self {
        Self {
            client: Client::builder()
                .pool_max_idle_per_host(max_sessions_per_moment)
                .pool_idle_timeout(Duration::from_secs(60))
                .build().unwrap(),
            max_connections_per_moment: max_sessions_per_moment,
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
    /// let max_connections = http_client.max_sessions_per_moment();
    /// ```
    pub fn max_connections_per_moment(&self) -> usize {
        self.max_connections_per_moment
    }
}
