use super::super::errors::*;

use {
    bytes::*,
    http::{header::*, response::*, *},
    std::result::Result,
};

/// Creates a response with an HTML body and provided headers.
///
/// The `Content-Type` and `Content-Length` headers will be set, overriding existing values.
///
/// The response body must implement [From]\<[Bytes]\>.
pub fn response_with_html_body<BodyT>(body: Bytes, headers: HeaderMap) -> Result<Response<BodyT>, StatusCode>
where
    BodyT: From<Bytes>,
{
    let mut builder = Response::builder();

    for (name, value) in headers {
        if let Some(name) = name {
            match name {
                CONTENT_TYPE | CONTENT_LENGTH => {}
                _ => builder = builder.header(name, value),
            };
        }
    }

    builder
        .header(CONTENT_TYPE, "text/html")
        .header(CONTENT_LENGTH, body.len())
        .body(body.into())
        .map_err_internal_server("build response")
}
