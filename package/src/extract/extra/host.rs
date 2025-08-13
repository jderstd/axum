use axum_core::extract::FromRequestParts;
use axum_extra::extract::Host as _Host;
use http::request::Parts;

use crate::response::{
    Response,
    json::{
        CreateJsonResponse, JsonResponseError, error::JsonResponseErrorCode,
    },
};

/// Extractor that resolves the hostname of the request,
/// available with `extra` feature.
///
/// Check [`Host`](axum_extra::extract::Host) for more information.
///
/// ## Example
///
/// ```no_run
/// use jder_axum::extract::extra::Host;
///
/// async fn route(
///     Host(host): Host
/// ) {
///     // ...
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Host(pub String);

impl<S> FromRequestParts<S> for Host
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        match _Host::from_request_parts(parts, state).await {
            | Ok(val) => Ok(Self(val.0)),
            | Err(rej) => Err(CreateJsonResponse::failure()
                .status(rej.status())
                .error(
                    JsonResponseError::builder()
                        .code(JsonResponseErrorCode::Parse.as_str())
                        .message(rej.body_text())
                        .build(),
                )
                .send()),
        }
    }
}
