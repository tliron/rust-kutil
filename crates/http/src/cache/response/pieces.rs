use super::super::body::*;

use {bytes::*, http::response::*};

//
// ResponsePieces
//

/// [Response] pieces.
///
/// Can be used to reconstruct a response, e.g. with
/// [BodyReader::new_with_first_bytes](super::super::super::body::BodyReader::new_with_first_bytes).
pub struct ResponsePieces<ResponseBodyT> {
    /// Response.
    pub response: Response<ResponseBodyT>,

    /// First bytes.
    pub first_bytes: Bytes,
}

impl<ResponseBodyT> ResponsePieces<ResponseBodyT> {
    /// Constructor.
    pub fn new(parts: Parts, body: ResponseBodyT, first_bytes: Bytes) -> Self {
        Self { response: Response::from_parts(parts, body), first_bytes }
    }

    /// Constructor.
    pub fn new_from(parts: Parts, body_pieces: BodyPieces<ResponseBodyT>) -> Self {
        Self::new(parts, body_pieces.body, body_pieces.first_bytes)
    }
}
