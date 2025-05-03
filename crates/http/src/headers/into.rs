use http::*;

//
// IntoHeaderValue
//

/// Into [HeaderValue].
pub trait IntoHeaderValue {
    /// To [HeaderValue].
    fn into_header_value(self) -> HeaderValue;
}

impl IntoHeaderValue for HeaderValue {
    fn into_header_value(self) -> HeaderValue {
        self
    }
}
