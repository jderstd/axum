use http::{HeaderMap, StatusCode, Version};

use crate::response::json::{
    error::JsonResponseError,
    functions::{
        failure::JsonFailureResponseFunctions,
        success::JsonSuccessResponseFunctions,
    },
};

/// Internal state.
#[derive(Debug, Clone)]
pub struct JsonResponseState<D> {
    pub status: StatusCode,
    pub version: Version,
    pub header_map: HeaderMap,
    pub is_header_map_failed: bool,
    pub success: bool,
    pub data: Option<D>,
    pub errors: Vec<JsonResponseError>,
}

impl<D> JsonResponseState<D> {
    /// Create a success JSON response state.
    pub fn success() -> Self {
        Self {
            status: StatusCode::OK,
            version: Version::HTTP_11,
            header_map: HeaderMap::new(),
            is_header_map_failed: false,
            success: true,
            data: None,
            errors: Vec::new(),
        }
    }

    /// Create a failure JSON response state.
    pub fn failure() -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            version: Version::HTTP_11,
            header_map: HeaderMap::new(),
            is_header_map_failed: false,
            success: false,
            data: None,
            errors: Vec::new(),
        }
    }
}

/// Create a JSON response for a route.
///
/// ## Examples
///
/// A success JSON response without data:
///
/// ```no_run
/// use jder_axum::response::{
///     Response,
///     json::CreateJsonResponse,
/// };
///
/// async fn route() -> Response {
///     CreateJsonResponse::dataless().create()
/// }
/// ```
///
/// A success JSON response:
///
/// ```no_run
/// use jder_axum::response::{
///     Response,
///     json::CreateJsonResponse,
/// };
/// use serde::Serialize;
///
/// #[derive(Default, Serialize)]
/// struct ResponseData {
///    name: String,
/// }
///
/// async fn route() -> Response {
///     CreateJsonResponse::success::<ResponseData>()
///         .data(ResponseData { name: "Name".to_string() })
///         .create()
/// }
/// ```
///
/// A failure JSON response:
///
/// ```no_run
/// use jder_axum::response::{
///     Response,
///     json::CreateJsonResponse,
/// };
///
/// async fn route() -> Response {
///     CreateJsonResponse::failure().create()
/// }
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct CreateJsonResponse;

impl CreateJsonResponse {
    /// Create a success JSON response without data.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use jder_axum::response::{
    ///     Response,
    ///     json::CreateJsonResponse,
    /// };
    ///
    /// async fn route() -> Response {
    ///     CreateJsonResponse::dataless().create()
    /// }
    /// ```
    pub fn dataless() -> JsonSuccessResponseFunctions<()> {
        JsonSuccessResponseFunctions { state: JsonResponseState::success() }
    }

    /// Create a success JSON response.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use jder_axum::response::{
    ///     Response,
    ///     json::CreateJsonResponse,
    /// };
    /// use serde::Serialize;
    ///
    /// #[derive(Default, Serialize)]
    /// struct ResponseData {
    ///    name: String,
    /// }
    ///
    /// async fn route() -> Response {
    ///     CreateJsonResponse::success::<ResponseData>()
    ///         .data(ResponseData { name: "Name".to_string() })
    ///         .create()
    /// }
    /// ```
    pub fn success<D>() -> JsonSuccessResponseFunctions<D> {
        JsonSuccessResponseFunctions { state: JsonResponseState::success() }
    }

    /// Create a failure JSON response.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use jder_axum::response::{
    ///     Response,
    ///     json::CreateJsonResponse,
    /// };
    ///
    /// async fn route() -> Response {
    ///     CreateJsonResponse::failure().create()
    /// }
    /// ```
    pub fn failure() -> JsonFailureResponseFunctions<()> {
        JsonFailureResponseFunctions { state: JsonResponseState::failure() }
    }
}
