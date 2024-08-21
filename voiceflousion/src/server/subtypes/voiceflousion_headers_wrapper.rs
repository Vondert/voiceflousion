use std::convert::Infallible;
use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::{HeaderMap, HeaderValue};
use axum::http::request::Parts;

/// A struct that wraps `HeaderMap` to provide convenient methods for accessing HTTP headers.
///
/// `VoiceflousionHeadersWrapper` encapsulates the HTTP headers, offering utility methods
/// to retrieve specific headers as `HeaderValue` or as `&str`.
pub struct VoiceflousionHeadersWrapper {
    headers: HeaderMap,
}

impl VoiceflousionHeadersWrapper {
    /// Creates a new `VoiceflousionHeadersWrapper` from a `HeaderMap`.
    ///
    /// # Parameters
    ///
    /// - `headers`: The `HeaderMap` containing the HTTP headers.
    ///
    /// # Returns
    ///
    /// A new instance of `VoiceflousionHeadersWrapper`.
    fn new(headers: HeaderMap) -> Self {
        Self { headers }
    }

    /// Returns a reference to the underlying `HeaderMap`.
    ///
    /// # Returns
    ///
    /// A reference to the `HeaderMap` stored within the wrapper.
    pub fn get_header_map(&self) -> &HeaderMap {
        &self.headers
    }

    /// Retrieves a specific header by name as an `Option<&HeaderValue>`.
    ///
    /// # Parameters
    ///
    /// - `header_name`: The name of the header to retrieve.
    ///
    /// # Returns
    ///
    /// An `Option<&HeaderValue>` containing the header value if found, or `None` if the header does not exist.
    pub fn get_header(&self, header_name: &str) -> Option<&HeaderValue> {
        self.headers.get(header_name)
    }

    /// Retrieves a specific header by name and attempts to convert it to a `&str`.
    ///
    /// # Parameters
    ///
    /// - `header_name`: The name of the header to retrieve.
    ///
    /// # Returns
    ///
    /// An `Option<&str>` containing the header value as a string slice if found and valid UTF-8, or `None` if the header does not exist or is not valid UTF-8.
    pub fn get_header_str(&self, header_name: &str) -> Option<&str> {
        self.get_header(header_name).and_then(|header| {
            header.to_str().ok()
        })
    }

    /// Retrieves a specific header by name and attempts to convert it to a `&str`, returning an empty string if not found or invalid.
    ///
    /// # Parameters
    ///
    /// - `header_name`: The name of the header to retrieve.
    ///
    /// # Returns
    ///
    /// A `&str` containing the header value as a string slice if found and valid UTF-8, or an empty string if the header does not exist or is not valid UTF-8.
    pub fn get_header_str_or_empty(&self, header_name: &str) -> &str {
        self.get_header_str(header_name).unwrap_or_default()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for VoiceflousionHeadersWrapper
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    /// Extracts `VoiceflousionHeadersWrapper` from the request parts.
    ///
    /// This method clones the headers from the `Parts` of the request and wraps them in a `VoiceflousionHeadersWrapper`.
    ///
    /// # Parameters
    ///
    /// - `parts`: The mutable reference to the `Parts` of the request, containing the headers.
    /// - `_state`: The state of the request, which is not used in this implementation.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `VoiceflousionHeadersWrapper` if successful, or an `Infallible` error type if failed.
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let headers = parts.headers.clone();
        Ok(VoiceflousionHeadersWrapper::new(headers))
    }
}
