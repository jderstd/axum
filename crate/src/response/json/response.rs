use serde::{Deserialize, Serialize};

use crate::response::json::error::JsonResponseError;

/// JSON response.
///
/// For API documentation generation with utoipa,
/// `ToSchema` derive is available with the `utoipa` feature.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct JsonResponse<D = ()> {
    /// Indicates whether the response is successful or not.
    pub success: bool,
    /// Requested information for the response when `success` is `true`.
    pub data: Option<D>,
    /// A list of errors for the response when `success` is `false`.
    pub errors: Vec<JsonResponseError>,
}

impl<D> JsonResponse<D> {
    /// Create a new JSON response.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use jder_axum::response::json::JsonResponse;
    ///
    /// let response: JsonResponse = JsonResponse::new();
    /// ```
    pub fn new() -> Self {
        Self { success: true, data: None, errors: vec![] }
    }

    /// Create a JSON response from another JSON response.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use jder_axum::response::json::JsonResponse;
    ///
    /// let response: JsonResponse = JsonResponse::new();
    ///
    /// let response: JsonResponse = JsonResponse::from(response);
    /// ```
    pub fn from<R: Into<JsonResponse<D>>>(response: R) -> Self {
        let res: JsonResponse<D> = response.into();

        Self { success: res.success, data: res.data, errors: res.errors }
    }

    /// Set the success status for the response.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use jder_axum::response::json::JsonResponse;
    ///
    /// let response: JsonResponse = JsonResponse::new()
    ///     .success(false);
    /// ```
    pub fn success(
        mut self,
        success: bool,
    ) -> Self {
        self.success = success;

        self
    }

    /// Set the data for the response.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use serde::{Serialize, Deserialize};
    /// use jder_axum::response::json::JsonResponse;
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize)]
    /// struct ResponseData {
    ///    name: String,
    /// }
    ///
    /// let response: JsonResponse<ResponseData> = JsonResponse::new()
    ///     .data(
    ///         ResponseData {
    ///             name: "Name".to_string()
    ///         }
    ///     );
    /// ```
    pub fn data(
        mut self,
        data: D,
    ) -> Self {
        self.data = Some(data);

        self
    }

    /// Set the errors for the response.
    ///
    /// This will overwrite any existing errors.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use jder_axum::response::json::{
    ///     JsonResponse,
    ///     JsonResponseError,
    /// };
    ///
    /// let error_name: JsonResponseError = JsonResponseError::new()
    ///     .code("parse")
    ///     .path(["json", "name"])
    ///     .message("Invalid name");
    ///
    /// let error_age: JsonResponseError = JsonResponseError::new()
    ///     .code("parse")
    ///     .path(["json", "age"])
    ///     .message("Invalid age");
    ///
    /// let response: JsonResponse = JsonResponse::new()
    ///     .errors([
    ///         error_name,
    ///         error_age,
    ///     ]);
    /// ```
    pub fn errors<V, E>(
        mut self,
        errors: V,
    ) -> Self
    where
        V: IntoIterator<Item = E>,
        E: Into<JsonResponseError>,
    {
        let errors: Vec<JsonResponseError> =
            errors.into_iter().map(|e| e.into()).collect();

        self.errors = errors;

        self
    }
}

impl Default for JsonResponse {
    fn default() -> Self {
        Self::new()
    }
}
