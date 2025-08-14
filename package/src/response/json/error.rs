use serde::{Deserialize, Serialize};

/// JSON response error code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsonResponseErrorCode {
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

impl JsonResponseErrorCode {
    // Create a new JSON response error code with default value.
    pub fn new() -> Self {
        Self::Unknown
    }

    /// Get the error code as `&str`.
    pub fn as_str(&self) -> &str {
        match self {
            | Self::Parse => "parse",
            | Self::TooLarge => "too_large",
            | Self::Timeout => "timeout",
            | Self::Server => "server",
            | Self::Unknown => "unknown",
        }
    }
}

impl Default for JsonResponseErrorCode {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for JsonResponseErrorCode {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub(crate) const FAILURE_RESPONSE_DEFAULT: &str = "{\"success\":false,\"data\":null,\"errors\":[{\"code\":\"server\",\"path\":[],\"message\":\"Internal server error.\"}]}";

/// A builder to create a JSON response error.
///
/// ## Example
///
/// ```no_run
/// use jder_axum::response::json::JsonResponseError;
///
/// let error: JsonResponseError = JsonResponseError::builder()
///     .code("parse")
///     .path(["json", "title"])
///     .message("Invalid title")
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct JsonResponseErrorBuilder {
    pub error: JsonResponseError,
}

impl JsonResponseErrorBuilder {
    /// Create a new JSON response error builder.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use jder_axum::response::json::JsonResponseError;
    ///
    /// let error: JsonResponseError = JsonResponseError::builder().build();
    /// ```
    pub fn new() -> Self {
        Self {
            error: JsonResponseError {
                code: JsonResponseErrorCode::new().to_string(),
                path: Vec::new(),
                message: None,
            },
        }
    }

    /// Set an error code for the response.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use jder_axum::response::json::JsonResponseError;
    ///
    /// let error: JsonResponseError = JsonResponseError::builder()
    ///     .code("parse")
    ///     .build();
    /// ```
    pub fn code<Code: Into<String>>(
        mut self,
        code: Code,
    ) -> Self {
        self.error.code = code.into();

        self
    }

    /// Set an error path for the response.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use jder_axum::response::json::JsonResponseError;
    ///
    /// let error: JsonResponseError = JsonResponseError::builder()
    ///     .path(["json", "title"])
    ///     .build();
    /// ```
    pub fn path<P, S>(
        mut self,
        path: P,
    ) -> Self
    where
        P: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.error.path = path.into_iter().map(|s| s.into()).collect();
        self
    }

    /// Set an error message for the response.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use jder_axum::response::json::JsonResponseError;
    ///
    /// let error: JsonResponseError = JsonResponseError::builder()
    ///     .message("Invalid title")
    ///     .build();
    /// ```
    pub fn message<Message: Into<String>>(
        mut self,
        message: Message,
    ) -> Self {
        self.error.message = Some(message.into());

        self
    }

    /// Build the JSON response error.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use jder_axum::response::json::JsonResponseError;
    ///
    /// let error: JsonResponseError = JsonResponseError::builder().build();
    /// ```
    pub fn build(self) -> JsonResponseError {
        self.error
    }
}

impl Default for JsonResponseErrorBuilder {
    fn default() -> Self {
        Self::new()
    }
}

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
            code: JsonResponseErrorCode::new().to_string(),
            path: Vec::new(),
            message: None,
        }
    }

    /// Create a JSON response error from an existing error.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use jder_axum::response::json::JsonResponseError;
    ///
    /// let error: JsonResponseError = JsonResponseError::from(JsonResponseError::new());
    /// ```
    pub fn from(error: JsonResponseError) -> Self {
        Self { code: error.code, path: error.path, message: error.message }
    }

    /// A builder function to create a JSON response error.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use jder_axum::response::json::JsonResponseError;
    ///
    /// let error: JsonResponseError = JsonResponseError::builder()
    ///     .code("parse")
    ///     .path(["json", "title"])
    ///     .message("Invalid title")
    ///     .build();
    /// ```
    pub fn builder() -> JsonResponseErrorBuilder {
        JsonResponseErrorBuilder::new()
    }
}

impl Default for JsonResponseError {
    fn default() -> Self {
        Self::new()
    }
}
