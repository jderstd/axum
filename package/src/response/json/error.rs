use serde::{Deserialize, Serialize};

/// Response error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseError {
    /// Error while parsing.
    Parse,
    /// Payload too large.
    TooLarge,
    /// Timeout error.
    Timeout,
    /// Internal server error.
    Server,
    /// Unknown error.
    Unknown,
}

impl ResponseError {
    // Create a new response error with default value.
    pub fn new() -> Self {
        Self::Unknown
    }

    /// Get the error code as `&str`.
    pub fn as_code(&self) -> &str {
        match self {
            | Self::Parse => "parse",
            | Self::TooLarge => "too_large",
            | Self::Timeout => "timeout",
            | Self::Server => "server",
            | Self::Unknown => "unknown",
        }
    }

    /// Get the error code as `String`.
    pub fn to_code(&self) -> String {
        self.as_code().to_string()
    }

    /// Get the error message as `&str`.
    pub fn as_message(&self) -> &str {
        match self {
            | Self::Parse => "Failed to parse the request",
            | Self::TooLarge => "Request body is too large",
            | Self::Timeout => "Gateway timeout",
            | Self::Server => "Internal server error",
            | Self::Unknown => "Unknown error",
        }
    }

    /// Get the error message as `String`.
    pub fn to_message(&self) -> String {
        self.as_message().to_string()
    }
}

impl Default for ResponseError {
    fn default() -> Self {
        Self::new()
    }
}

pub const FAILURE_RESPONSE_DEFAULT: &str = "{\"success\":false,\"data\":null,\"errors\":[{\"code\":\"server\",\"path\":[],\"message\":\"Internal server error.\"}]}";

/// JSON response error.
///
/// For API documentation generation with utoipa,
/// `ToSchema` derive is available with the `utoipa` feature.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct JsonResponseError {
    /// Code representing the error.
    pub code: String,
    /// Indicates where the error occurred.
    pub path: Vec<String>,
    /// Detail of the error.
    pub message: Option<String>,
}

impl JsonResponseError {
    /// Create a new JSON response error.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use jder_axum::response::json::JsonResponseError;
    ///
    /// let error: JsonResponseError = JsonResponseError::new();
    /// ```
    pub fn new() -> Self {
        Self {
            code: ResponseError::new().to_code(),
            path: Vec::new(),
            message: None,
        }
    }

    /// Create a new JSON response error from an existing error.
    /// 
    /// ## Example
    ///
    /// ```no_run
    /// use jder_axum::response::json::JsonResponseError;
    ///
    /// let error: JsonResponseError = JsonResponseError::new();
    /// 
    /// let error: JsonResponseError = JsonResponseError::from(error);
    /// ```
    pub fn from<E: Into<JsonResponseError>>(error: E) -> Self {
        let err: JsonResponseError = error.into();

        Self {
            code: err.code,
            path: err.path,
            message: err.message,
        }
    }

    /// A builder function to create a JSON response error.
    #[deprecated = "Use `new` function instead"]
    pub fn builder() -> Self {
        Self::new()
    }

    /// Set an error code for the response.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use jder_axum::response::json::JsonResponseError;
    ///
    /// let error: JsonResponseError = JsonResponseError::new()
    ///     .code("parse");
    /// ```
    pub fn code<Code: Into<String>>(
        mut self,
        code: Code,
    ) -> Self {
        self.code = code.into();

        self
    }

    /// Set an error path for the response.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use jder_axum::response::json::JsonResponseError;
    ///
    /// let error: JsonResponseError = JsonResponseError::new()
    ///     .path(["json", "title"]);
    /// ```
    pub fn path<P, S>(
        mut self,
        path: P,
    ) -> Self
    where
        P: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.path = path.into_iter().map(|s| s.into()).collect();
        self
    }

    /// Set an error message for the response.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use jder_axum::response::json::JsonResponseError;
    ///
    /// let error: JsonResponseError = JsonResponseError::new()
    ///     .message("Invalid title");
    /// ```
    pub fn message<Message: Into<String>>(
        mut self,
        message: Message,
    ) -> Self {
        self.message = Some(message.into());

        self
    }

    /// Build the JSON response error.
    #[deprecated = "No longer needed"]
    pub fn build(self) -> Self {
        self
    }
}

impl Default for JsonResponseError {
    fn default() -> Self {
        Self::new()
    }
}
