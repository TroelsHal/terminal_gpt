/// Custom errors for the terminal chat application.
///
/// This enum covers a range of errors encountered during operations such as
/// serialization, network requests, response handling, and JSON parsing.
#[derive(Debug)]
pub enum CustomError {
    /// Error in serialization (e.g., to JSON).
    Serialization(serde_json::Error),

    /// Error when making an HTTP request.
    Request(reqwest::Error),

    /// Non-200 HTTP response status code.
    Response(reqwest::StatusCode),

    /// Error reading the response body from an HTTP request.
    ReadResponse(reqwest::Error),

    /// Error parsing the JSON in the response body.
    ParseJson(serde_json::Error),

    /// No message found in the API response.
    NoMessageFound,
}
