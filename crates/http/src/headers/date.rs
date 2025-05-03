use super::into::*;

use {http::*, httpdate::*, std::time::*};

/// Current time as [HttpDate].
pub fn now() -> HttpDate {
    HttpDate::from(SystemTime::now())
}

/// Whether we have been modified since a reference date.
///
/// If there is not enough information we will assume that we have been modified and return true.
pub fn modified_since(modified_date: Option<HttpDate>, reference_date: Option<HttpDate>) -> bool {
    if let Some(last_modified) = modified_date {
        if let Some(reference_date) = reference_date {
            if last_modified <= reference_date {
                return false;
            }
        }
    }

    true
}

impl IntoHeaderValue for HttpDate {
    fn into_header_value(self) -> HeaderValue {
        HeaderValue::from_str(&fmt_http_date(self.into()))
            .expect("fmt_http_date should always create a valid HeaderValue string")
    }
}
