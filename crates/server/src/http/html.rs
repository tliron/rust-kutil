use super::errors::*;

use http::{HeaderMap, StatusCode, header::*, response::*};

/// Creates a response with an HTML body and provided headers.
///
/// The "Content-Type" and "Content-Length" headers will be set, overriding provided values.
pub fn html_response<BodyT>(body: String, headers: HeaderMap) -> Result<Response<BodyT>, StatusCode>
where
    BodyT: From<String>,
{
    let mut builder = Response::builder();

    for (name, value) in headers {
        //println!("header: {:?} = {:?}", name, value);
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
