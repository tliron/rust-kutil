use super::{super::super::body::*, pieces::*};

use {::bytes::*, http_body::*, kutil_std::error::*, std::io};

//
// ReadBodyIntoBytesBounded
//

/// Read [Body] into [Bytes] within bounds.
#[allow(async_fn_in_trait)]
pub trait ReadBodyIntoBytesBounded<BodyT> {
    /// Read [Body] into [Bytes].
    ///
    /// If `known_size` is not [None](Option::None) then that's the size we expect. Otherwise
    /// we'll try to read to `max_size` and will expect at least `min_size`.
    ///
    /// In either case we will return an error if the body wasn't completely read (we won't cache
    /// incomplete bodies!), together with [BodyPieces], which can be used by the caller to
    /// reconstruct the original body.
    async fn read_into_bytes_bounded(
        self,
        known_size: Option<usize>,
        min_size: usize,
        max_size: usize,
    ) -> Result<Bytes, (CapturedError, Option<BodyPieces<BodyT>>)>;
}

impl<BodyT> ReadBodyIntoBytesBounded<BodyT> for BodyT
where
    BodyT: Body + Unpin,
    BodyT::Error: Into<CapturedError>,
{
    async fn read_into_bytes_bounded(
        self,
        known_size: Option<usize>,
        min_size: usize,
        max_size: usize,
    ) -> Result<Bytes, (CapturedError, Option<BodyPieces<BodyT>>)> {
        assert!(max_size >= min_size);

        let size = match known_size {
            Some(known_size) => {
                assert!(known_size >= min_size);
                assert!(known_size <= max_size);
                known_size
            }

            None => max_size,
        };

        match self.read_into_bytes(size).await {
            Ok((body, complete, bytes, _trailers)) => {
                let fulfilled_size = bytes.len();

                if complete {
                    match known_size {
                        Some(content_length) => {
                            if content_length != fulfilled_size {
                                // This situation is OK with us!
                                tracing::warn!(
                                    "Content-Length is {} but actual body size is smaller: {}",
                                    content_length,
                                    fulfilled_size
                                );
                            }

                            Ok(bytes)
                        }

                        None => {
                            if fulfilled_size >= min_size {
                                Ok(bytes)
                            } else {
                                Err((
                                    io::Error::other(format!(
                                        "body is too big to cache: {} > {}",
                                        fulfilled_size, min_size
                                    ))
                                    .into(),
                                    Some(BodyPieces::new(body, bytes)),
                                ))
                            }
                        }
                    }
                } else {
                    Err((
                        io::Error::other(format!("will not cache incompletely read body: {}", fulfilled_size)).into(),
                        Some(BodyPieces::new(body, bytes)),
                    ))
                }
            }

            Err(error) => return Err((error, None)),
        }
    }
}
