use super::{super::encoding::*, qvalue::*};

use http::header::*;

//
// AcceptedEncodings
//

/// Accepted encodings.
#[allow(unused)]
pub struct AcceptedEncodings(Vec<WeightedEncoding>);

impl AcceptedEncodings {
    /// Constructor.
    ///
    /// Based on:
    /// https://github.com/tower-rs/tower-http/blob/main/tower-http/src/content_encoding.rs
    /// Which is based on:
    /// https://github.com/http-rs/accept-encoding/blob/master/src/lib.rs
    pub fn new(request_headers: &HeaderMap) -> Self {
        let mut weighted_encodings: Vec<_> = request_headers
            .get_all(ACCEPT_ENCODING)
            .iter()
            .filter_map(|value| value.to_str().ok())
            .flat_map(|string| string.split(','))
            .filter_map(move |encoding_string| {
                let mut split = encoding_string.splitn(2, ';');

                let encoding = match Encoding::parse(split.next().unwrap().trim()) {
                    Some(encoding) => encoding,
                    None => return None, // ignore unknown encodings
                };

                let weight = match split.next() {
                    Some(weight) => QValue::parse(weight.trim())?,
                    None => MAX_QVALUE,
                };

                Some(WeightedEncoding { encoding, weight })
            })
            .collect();

        weighted_encodings.sort_by(|a, b| b.weight.cmp(&a.weight));
        Self(weighted_encodings)
    }

    /// Best encoding.
    pub fn best(&self) -> Encoding {
        match self.0.len() {
            0 => Encoding::Identity,

            1 => self.0.first().unwrap().encoding.clone(),

            _ => {
                let preferred = self.preferred();

                let has = |encoding: &Encoding| -> bool {
                    for preferred_encoding in &preferred {
                        if preferred_encoding == encoding {
                            return true;
                        }
                    }
                    false
                };

                // Try encoding in order of *our* preference
                if has(&Encoding::Brotli) {
                    // Brotli is best
                    Encoding::Brotli
                } else if has(&Encoding::GZip) {
                    // GZip is better than Deflate because it allows clients to check for errors
                    Encoding::GZip
                } else if has(&Encoding::Deflate) {
                    Encoding::Deflate
                } else {
                    Encoding::Identity
                }
            }
        }
    }

    /// Preferred encodings (that have the max weight).
    pub fn preferred(&self) -> Vec<Encoding> {
        let mut highest = Vec::new();
        let max = self.max_weight();
        for weighted_encoding in &self.0 {
            if weighted_encoding.weight == max {
                highest.push(weighted_encoding.encoding.clone());
            }
        }
        highest
    }

    /// Max weight.
    pub fn max_weight(&self) -> QValue {
        let mut max = QValue(0);
        for weighted_encoding in &self.0 {
            if weighted_encoding.weight > max {
                max = weighted_encoding.weight;
            }
        }
        max
    }
}
