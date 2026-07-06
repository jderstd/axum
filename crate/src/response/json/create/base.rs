use axum_core::{body::Body, response::Response};
use http::{HeaderMap, HeaderValue, StatusCode, header, response::Builder};
use serde::Serialize;

use crate::response::json::{
    create::JsonResponseState,
    error::{FAILURE_RESPONSE_DEFAULT, JsonResponseError, ResponseError},
    response::JsonResponse,
};

/// JSON content type.
const CONTENT_TYPE_JSON: &str = "application/json";

pub fn create_json_response_fn<D: Serialize>(
    state: JsonResponseState<D>
) -> Response {
    // a server error that supposed to be always work
    let server_error: Response = Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header(header::CONTENT_TYPE, CONTENT_TYPE_JSON)
        .body(Body::from(FAILURE_RESPONSE_DEFAULT.to_string()))
        .unwrap();

    // header map error
    if state.is_header_map_failed {
        // create error
        let res: JsonResponse<D> = JsonResponse::new().success(false).errors([
            JsonResponseError::new()
                .code(ResponseError::Parse.to_code())
                .path(["response", "header_map"])
                .message("Failed to create header map."),
        ]);

        // parse body
        let body: String = match serde_json::to_string(&res) {
            | Ok(body) => body,
            | Err(_) => return server_error,
        };

        return match Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header(header::CONTENT_TYPE, CONTENT_TYPE_JSON)
            .body(Body::from(body))
        {
            | Ok(res) => res,
            | Err(_) => server_error,
        };
    }

    // create response builder
    let mut builder: Builder =
        Response::builder().status(state.status).version(state.version);

    // set content type
    let mut header_map: HeaderMap = state.header_map;

    header_map.append(
        header::CONTENT_TYPE,
        match HeaderValue::from_str(CONTENT_TYPE_JSON) {
            | Ok(value) => value,
            | Err(_) => return server_error,
        },
    );

    // push headers
    for (header, value) in header_map {
        if let Some(header) = header {
            builder = builder.header(header, value);
        }
    }

    // create response
    let res: JsonResponse<D> = JsonResponse {
        success: state.success,
        data: state.data,
        errors: state.errors,
    };

    // parse body
    let body: String = match serde_json::to_string(&res) {
        | Ok(body) => body,
        | Err(_) => return server_error,
    };

    // result
    match builder.body(Body::from(body)) {
        | Ok(res) => res,
        | Err(_) => server_error,
    }
}
