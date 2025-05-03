use bytes::*;

//
// BodyPieces
//

/// [Body] pieces.
///
/// Can be used to reconstruct a body, e.g. with
/// [BodyReader::new_with_first_bytes](super::super::super::body::BodyReader::new_with_first_bytes).
pub struct BodyPieces<BodyT> {
    /// Body.
    pub body: BodyT,

    /// First bytes.
    pub first_bytes: Bytes,
}

impl<BodyT> BodyPieces<BodyT> {
    /// Constructor.
    pub fn new(body: BodyT, first_bytes: Bytes) -> Self {
        Self { body, first_bytes }
    }
}
