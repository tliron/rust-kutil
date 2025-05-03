use super::super::super::super::headers::*;

use {
    axum::{
        http::header::*,
        response::{IntoResponse, Response},
    },
    duration_str::*,
    std::{result::Result, time::*},
};

//
// Headers
//

/// Headers.
pub trait Headers<IntoResponseT>: Sized
where
    IntoResponseT: IntoResponse,
{
    /// Set `XX-Encode` header to "false".
    fn do_not_encode(self) -> Response {
        self.set_header_bool(ENCODE_HEADER, false)
    }

    /// Set `XX-Cache` header to "false".
    fn do_not_cache(self) -> Response {
        self.set_header_bool(CACHE_HEADER, false)
    }

    /// Set `XX-Cache-Duration` header.
    fn with_duration(self, duration: Duration) -> Response;

    /// Set `XX-Cache-Duration` header.
    fn with_duration_str(self, duration: &str) -> Result<Response, InvalidHeaderValue>;

    /// Set a header to a boolean value.
    fn set_header_bool(self, name: HeaderName, value: bool) -> Response;
}

impl<IntoResponseT> Headers<IntoResponseT> for IntoResponseT
where
    IntoResponseT: IntoResponse,
{
    fn with_duration(self, duration: Duration) -> Response {
        let mut response = self.into_response();
        let headers = response.headers_mut();
        headers.remove(CACHE_DURATION_HEADER);
        let duration = HeaderValue::from_str(duration.human_format().as_str())
            .expect("duration_str::HumanFormat should not return an invalid string");
        headers.set_value(CACHE_DURATION_HEADER, duration);
        response
    }

    fn with_duration_str(self, duration: &str) -> Result<Response, InvalidHeaderValue> {
        let mut response = self.into_response();
        let headers = response.headers_mut();
        headers.set_string_value(CACHE_DURATION_HEADER, duration)?;
        Ok(response)
    }

    fn set_header_bool(self, name: HeaderName, value: bool) -> Response {
        let mut response = self.into_response();
        response.headers_mut().set_bool_value(name, value);
        response
    }
}
