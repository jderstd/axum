use http::{Error as HTTPError, HeaderName, HeaderValue, StatusCode, Version};
use serde::Serialize;

use crate::response::{
    Response,
    json::{
        create::JsonResponseState, error::JsonResponseError,
        functions::create::create_json_response_fn,
    },
};

/// Functions for creating an failure response.
#[derive(Debug, Clone)]
pub struct JsonFailureResponseFunctions<D> {
    pub(crate) state: JsonResponseState<D>,
}

impl<D: Serialize> JsonFailureResponseFunctions<D> {
    /// Set the status code for the response.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use axum::http::StatusCode;
    /// use jder_axum::response::{
    ///     Response,
    ///     json::CreateJsonResponse
    /// };
    ///
    /// async fn route() -> Response {
    ///     CreateJsonResponse::failure()
    ///         .status(StatusCode::NOT_FOUND)
    ///         .create()
    /// }
    /// ```
    pub fn status<S: Into<StatusCode>>(
        mut self,
        status: S,
    ) -> Self {
        self.state.status = status.into();

        self
    }

    /// Set the HTTP version for the response.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use axum::http::Version;
    /// use jder_axum::response::{
    ///     Response,
    ///     json::CreateJsonResponse
    /// };
    ///
    /// async fn route() -> Response {
    ///     CreateJsonResponse::failure()
    ///         .version(Version::HTTP_3)
    ///         .create()
    /// }
    /// ```
    pub fn version<V: Into<Version>>(
        mut self,
        version: V,
    ) -> Self {
        self.state.version = version.into();

        self
    }

    /// Add a header for the response.
    ///
    /// For validation on key value, see
    /// [`get_header_from_key_value`](crate::response::header::get_header_from_key_value).
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use axum::http::{
    ///     header,
    ///     HeaderName,
    /// };
    /// use jder_axum::response::{
    ///     Response,
    ///     json::CreateJsonResponse,
    /// };
    ///
    /// async fn route() -> Response {
    ///     CreateJsonResponse::failure()
    ///         .header(
    ///             header::CONTENT_TYPE,
    ///             "application/json"
    ///         )
    ///         .create()
    /// }
    /// ```
    pub fn header<K, V>(
        mut self,
        key: K,
        value: V,
    ) -> Self
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<HTTPError>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<HTTPError>,
    {
        let key: HeaderName = match <HeaderName as TryFrom<K>>::try_from(key) {
            | Ok(k) => k,
            | Err(_) => {
                self.state.is_header_map_failed = true;
                return self;
            },
        };

        let value: HeaderValue =
            match <HeaderValue as TryFrom<V>>::try_from(value) {
                | Ok(v) => v,
                | Err(_) => {
                    self.state.is_header_map_failed = true;
                    return self;
                },
            };

        self.state.header_map.try_append(key, value).unwrap();

        self
    }

    /// Add multiple headers for the response.
    ///
    /// For validation on key value, see
    /// [`get_header_from_key_value`](crate::response::header::get_header_from_key_value).
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use axum::http::{
    ///     header,
    ///     HeaderName,
    /// };
    /// use jder_axum::response::{
    ///     Response,
    ///     json::CreateJsonResponse,
    /// };
    ///
    /// async fn route() -> Response {
    ///     let headers: Vec<(HeaderName, &str)> = vec![
    ///         (
    ///             header::CONTENT_TYPE,
    ///             "application/json"
    ///         ),
    ///         (
    ///             header::ACCESS_CONTROL_ALLOW_ORIGIN,
    ///             "*"
    ///         ),
    ///     ];
    ///
    ///     CreateJsonResponse::dataless()
    ///         .headers(headers)
    ///         .create()
    /// }
    /// ```
    pub fn headers<K, V>(
        mut self,
        headers: impl IntoIterator<Item = (K, V)>,
    ) -> Self
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<HTTPError>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<HTTPError>,
    {
        for (key, value) in headers {
            self = self.header(key, value);
        }

        self
    }

    /// Finish the response creation.
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
    pub fn create(self) -> Response {
        create_json_response_fn(self.state)
    }

    /// Finish the response creation.
    #[deprecated = "Use `create` instead"]
    pub fn send(self) -> Response {
        self.create()
    }
}

impl<D: Serialize> JsonFailureResponseFunctions<D> {
    /// Set errors for the response.
    ///
    /// This will override any existing errors.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use jder_axum::response::{
    ///     Response,
    ///     json::{
    ///         JsonResponseError,
    ///         CreateJsonResponse,
    ///     },
    /// };
    ///
    /// async fn route() -> Response {
    ///     let error_name: JsonResponseError = JsonResponseError::new()
    ///         .code("parse")
    ///         .path(["json", "name"])
    ///         .message("Invalid name");
    ///
    ///     let error_age: JsonResponseError = JsonResponseError::new()
    ///         .code("parse")
    ///         .path(["json", "age"])
    ///         .message("Invalid age");
    ///
    ///     CreateJsonResponse::failure()
    ///         .errors([
    ///             error_name,
    ///             error_age,
    ///         ])
    ///         .create()
    /// }
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

        self.state.errors = errors;

        self
    }

    /// Add multiple errors to the response.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use jder_axum::response::{
    ///     Response,
    ///     json::{
    ///         JsonResponseError,
    ///         CreateJsonResponse,
    ///     },
    /// };
    ///
    /// async fn route() -> Response {
    ///     let error_name: JsonResponseError = JsonResponseError::new()
    ///         .code("parse")
    ///         .path(["json", "name"])
    ///         .message("Invalid name");
    ///
    ///     let error_age: JsonResponseError = JsonResponseError::new()
    ///         .code("parse")
    ///         .path(["json", "age"])
    ///         .message("Invalid age");
    ///
    ///     CreateJsonResponse::failure()
    ///         .add_errors([
    ///             error_name,
    ///             error_age,
    ///         ])
    ///         .create()
    /// }
    /// ```
    pub fn add_errors<V, E>(
        mut self,
        errors: V,
    ) -> Self
    where
        V: IntoIterator<Item = E>,
        E: Into<JsonResponseError>,
    {
        self.state.errors.extend(errors.into_iter().map(|e| e.into()));

        self
    }

    /// Add an error to the response.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use jder_axum::response::{
    ///     Response,
    ///     json::{
    ///         JsonResponseError,
    ///         CreateJsonResponse,
    ///     },
    /// };
    ///
    /// async fn route() -> Response {
    ///     let error: JsonResponseError = JsonResponseError::new()
    ///         .code("parse")
    ///         .path(["json", "title"])
    ///         .message("Invalid title");
    ///
    ///     CreateJsonResponse::failure()
    ///         .add_error(error)
    ///         .create()
    /// }
    /// ```
    pub fn add_error(
        mut self,
        error: JsonResponseError,
    ) -> Self {
        self.state.errors.push(error);

        self
    }

    /// Add an error to the response.
    #[deprecated = "Use `add_error` instead"]
    pub fn error(
        mut self,
        error: JsonResponseError,
    ) -> Self {
        self.state.errors.push(error);

        self
    }
}
