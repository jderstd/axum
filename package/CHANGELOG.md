## Next

### Breaking Changes

- rename `JsonSuccessResponseFunctions` to `CreateSuccessJsonResponse`
- rename `JsonFailureResponseFunctions` to `CreateFailureJsonResponse`

## 0.10.1 (2025-12-16)

### What's Changed

- update documentation

## 0.10.0 (2025-12-16)

### Breaking Changes

Response:

- `CreateResponse` requires `create()` function to finish response creation now

JSON response:

- remove `JsonResponseErrorBuilder` struct

- deprecate `error` function in `CreateJsonResponse`
- deprecate `send` function in response creation
- deprecate `builder`, `build` function in `JsonResponseError`

- `errors` function of failure response overwrite existing errors now

Extractors:

- update result type of `from_bytes` in JSON extractor

### What's New

- add builder functions for `JsonResponse`
- add `from` function for `JsonResponseError`
- add `add_errors`, `add_error` function for failure response
- add `create` function for response creation

### Migrating from 0.9.X to 0.10.0

For creating the response:

```diff
use jder_axum::response::{
    Response,
    CreateResponse,
};

async fn route() -> Response {
-   CreateResponse::success().body("active")
+   CreateResponse::success()
+       .body("active")
+       .create()
}
```

For creating the JSON response:

```diff
use jder_axum::response::{
    Response,
    json::CreateJsonResponse,
};

async fn route() -> Response {
-    CreateJsonResponse::dataless().send()
+    CreateJsonResponse::dataless().create()
}
```

For creating the JSON response error:

```diff
use jder_axum::response::json::{
    JsonResponseError,
}

- let error: JsonResponseError = JsonResponseError::builder()
+ let error: JsonResponseError = JsonResponseError::new()
    .code("parse")
    .path(["json", "title"])
-   .message("Invalid title")
-   .build();
+   .message("Invalid title");
```

For creating the JSON response with error

```diff
use jder_axum::response::{
    Response,
    json::{
        JsonResponseError,
        CreateJsonResponse,
    },
};

async fn route() -> Response {
-   let error: JsonResponseError = JsonResponseError::builder().build();
+   let error: JsonResponseError = JsonResponseError::new();

    CreateJsonResponse::failure()
-       .error(error)
+       .add_error(error)
-       .send();
+       .create();
}
```

## 0.9.1 (2025-11-26)

### What's Changed

- update `axum-extra` supported version from `~0.10.1` to `~0.12.0`

## 0.9.0 (2025-09-20)

### Breaking Changes

- status code changed in `RequestTimeLimit` layer:
    - `408` -> `504`

### What's Changed

- update documentation

## 0.8.0 (2025-09-06)

### Breaking Changes

- rename `JsonResponseErrorCode` to `ResponseError`

### What's New

- add `new` functions for `JsonResponseError`
- add `Default` derive for `JsonResponseError`
- add `new` functions for `ResponseError`
- add `Default` derive for `ResponseError`

### What's Changes

- optimize internal functions

## 0.7.0 (2025-08-13)

### Breaking Changes

- support new standard

### What's New

- add `builder` function for `JsonResponseError`

### Migrating from 0.6.X to 0.7.0

```diff
use jder_axum::response::{
    Response,
    json::{
        CreateJsonResponse,
        JsonResponseError,
    },
};

async fn route() -> Response {
    CreateJsonResponse::failure()
-       .error_code("parse")
-       .error_field("title")
-       .error_message("Invalid title")
+       .error(
+           JsonResponseError::builder()
+               .code("parse")
+               .path(["json", "title"])
+               .message("Invalid title")
+               .build(),
+       )
        .send()
}
```

## 0.6.1 (2025-07-13)

### What's Changed

- add missing default constructor for `RequestBodyLimit`
- add missing default constructor for `RequestTimeLimit`

## 0.6.0 (2025-05-20)

### What's New

- add `RequestTimeLimit` layer

### What's Changed

- remove unnecessary implementation in `RequestBodyLimit` layer

## 0.5.0 (2025-05-13)

### Breaking Changes

- move form module to `form` feature
- move json module to `json` feature
- move host module to `extra` feature
- move matched path module to `matched_path` feature
- move multipart module to `typed_multipart` feature
- move query module to `query` feature
- move connect info module to `tokio` feature
- move host module to `extra` feature
- rename `empty_to_none` to `empty_as_none`
- rename `Multipart` to `TypedMultipart`
- rename `MultipartFailureResponse` to `TypedMultipartFailureResponse`
- remove `State` as it is infallible
- remove `OriginalUri` as it is infallible

### What's New

- add support for optional request
- add `Form` extractor
- add `Scheme` extractor
- add `TypedHeader` extractor
- add `RequestBodyLimit` layer
- add features:
    - `form`
    - `json`
    - `matched_path`
    - `multipart`
    - `typed_multipart`
    - `query`
    - `tokio`
    - `request_body_limit`
    - `extra`
    - `extra_typed_header`
    - `utoipa`
- add new derive to struct for `utoipa` feature:
    - `JsonResponse`
    - `JsonResponseError`

### What's Changed

- update minimum `axum` version to `0.8.3`
- update to 2024 edition

### Migrating from 0.4.0 to 0.5.0

Update `Cargo.toml`:

```diff
[dependencies]
- jder_axum = "0.4.0"
+ jder_axum = { version = "0.5.0", features = ["extra", "typed_multipart"] }
```

Update `Host` extractor path:

```diff
- use jder_axum::extract::Host;
+ use jder_axum::extract::extra::Host;
```

Update path of `TypedMultipart` and `TypedMultipartFailureResponse`:

```diff
- use jder_axum::extract::Multipart;
+ use jder_axum::extract::multipart::TypedMultipart;

- use jder_axum::extract::MultipartFailureResponse;
+ use jder_axum::extract::multipart::typed::TypedMultipartFailureResponse;
```

Use extractors from `axum`:

```diff
- use jder_axum::extract::State;
+ use axum::extract::State;

- use jder_axum::extract::OriginalUri;
+ use axum::extract::OriginalUri;
```

Update function name:

```diff
- use jder_axum::extract::query::empty_to_none;
+ use jder_axum::extract::query::empty_as_none;
```

## 0.4.0 (2025-01-09)

### What's Changed

- add support for axum 0.8
- update dependencies

## 0.3.1 (2024-12-16)

### What's Changed

- update dependencies
- update documentation

## 0.3.0 (2024-10-26)

### Breaking Changes

- values renamed in `JsonResponseErrorCode`

### What's Changed

- `status` in `CreateJsonResponse` accept more types of input now
- `version` in `CreateJsonResponse` accept more types of input now
- `error_code` in `CreateJsonResponse` accept more types of input now
- `error_field` in `CreateJsonResponse` accept more types of input now
- `error_message` in `CreateJsonResponse` accept more types of input now

## 0.2.0 (2024-10-14)

### Breaking Changes

- enum `ResponseErrorCode` renamed to `JsonResponseErrorCode`
- values renamed in `JsonResponseErrorCode`:
    - `ParseError` => `Parse`
    - `ServerError` => `Server`
    - `UnknownError` => `Unknown`
- move JSON related stuffs into `response::json` module
- changes in accepted value type of `error_code`:
    - `String` => `&str`
- changes in accepted value type of `error_field`:
    - `String` => `&str`
- changes in accepted value type of `error_message`:
    - `String` => `&str`

### What's New

- add different derives for different structs
- add `get_header_from_key_value` function
- add `get_header_name_from_key` function
- add `get_header_value_from_value` function
- add `as_str` for `JsonResponseErrorCode`

### What's Changed

- changes in accepted value type of `key` in `header` and `headers`:
    - `HeaderName` => `&str`/`String`/`HeaderName`
- changes in accepted value type of `value` in `header` and `headers`:
    - `String` => `&str`/`String`/`HeaderValue`
- updates in documentation

## 0.1.0 (2024-10-04)

initial release
