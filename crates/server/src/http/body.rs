use super::errors::*;

use {bytes::*, http_body::*, http_body_util::*};

//
// BodyToBytes
//

/// Body to bytes.
#[allow(async_fn_in_trait)]
pub trait BodyToBytes {
    /// Body to bytes.
    async fn to_bytes(self, max_size: usize) -> Result<Bytes, BoxedError>;

    /// Body to string.
    async fn to_string(self, max_size: usize) -> Result<String, BoxedError>;
}

impl<BodyT> BodyToBytes for BodyT
where
    BodyT: Body,
    BodyT::Error: Into<BoxedError>,
{
    async fn to_bytes(self, max_size: usize) -> Result<Bytes, BoxedError> {
        Limited::new(self, max_size).collect().await.map(|collected| collected.to_bytes())
    }

    async fn to_string(self, max_size: usize) -> Result<String, BoxedError> {
        let bytes = self.to_bytes(max_size).await?;
        Ok(String::from_utf8(bytes.to_vec())?)
    }
}
