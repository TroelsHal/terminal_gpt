use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client, Error,
};

/// A builder for creating an HTTP client.
///
/// This builder is responsible for creating a `reqwest` client configured with
/// necessary headers such as Authorization for API key access and Content-Type for JSON requests.
pub struct ClientBuilder {
    api_key: String,
}

/// Creates a new `ClientBuilder` instance.
///
/// # Arguments
///
/// * `api_key` - A string slice that holds the API key for authorization.
impl ClientBuilder {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
        }
    }

    /// Builds and returns a `reqwest` client.
    ///
    /// This method sets up the client with all necessary default headers.
    /// It returns a `Result` which, on success, contains the `reqwest` client,
    /// or an error if the build fails.
    ///
    /// # Errors
    ///
    /// This method will return an error if the client fails to build.
    pub fn build(&self) -> Result<Client, Error> {
        let mut headers: HeaderMap = HeaderMap::new();
        let header_string = format!("Bearer {}", self.api_key);
        // AUTHORIZATION header used for passing the API key.
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&header_string).unwrap(),
        );
        // CONTENT_TYPE header set to indicate that the body of the HTTP
        // request is formatted as JSON.
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        Client::builder().default_headers(headers).build()
    }
}
