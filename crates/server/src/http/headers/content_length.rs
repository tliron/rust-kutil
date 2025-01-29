use http::{HeaderMap, header::*};

/// Get Content-Length.
pub fn get_content_length_header(headers: &HeaderMap) -> Option<usize> {
    match headers.get(CONTENT_LENGTH) {
        Some(length) => match length.to_str() {
            Ok(length) => match length.parse::<usize>() {
                Ok(length) => Some(length),

                Err(error) => {
                    tracing::warn!("invalid Content-Length: {}", error);
                    None
                }
            },

            Err(error) => {
                tracing::warn!("invalid Content-Length: {}", error);
                None
            }
        },

        None => None,
    }
}
