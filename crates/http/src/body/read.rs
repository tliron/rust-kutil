use super::reader::*;

use {
    bytes::*,
    http::*,
    http_body::*,
    kutil_std::error::*,
    std::{io, result::Result},
    tokio::io::*,
};

//
// ReadBodyIntoBytes
//

/// Read [Body] into [Bytes].
#[allow(async_fn_in_trait)]
pub trait ReadBodyIntoBytes: Sized {
    /// Read [Body] into [Bytes] and trailers.
    ///
    /// Returns true if we read all the way to EOF.
    ///
    /// If EOF was not reached then perhaps the caller could reconstruct the body by pushing
    /// the bytes we read to its front, e.g. with
    /// [BodyReader::new_with_first_bytes](super::reader::BodyReader::new_with_first_bytes).
    async fn read_into_bytes(self, max_size: usize) -> Result<(Self, bool, Bytes, Vec<HeaderMap>), CapturedError>;

    /// Read [Body] into [String] and trailers.
    ///
    /// If we did not read all the way to EOF will return a [io::ErrorKind::FileTooLarge] error.
    async fn read_into_string(self, max_size: usize) -> Result<(String, Vec<HeaderMap>), CapturedError> {
        let (_, complete, bytes, trailers) = self.read_into_bytes(max_size).await?;
        if complete {
            Ok((String::from_utf8(bytes.to_vec())?, trailers))
        } else {
            Err(io::Error::new(io::ErrorKind::FileTooLarge, format!("max_size is {}", max_size)).into())
        }
    }
}

impl<BodyT> ReadBodyIntoBytes for BodyT
where
    BodyT: Body + Unpin,
    BodyT::Error: Into<CapturedError>,
{
    async fn read_into_bytes(self, max_size: usize) -> Result<(Self, bool, Bytes, Vec<HeaderMap>), CapturedError> {
        let mut reader = self.into_reader();

        let mut bytes = BytesMut::with_capacity(max_size);
        reader.read_buf(&mut bytes).await?;

        // Do we have any more data?
        // We'll try to read just one more byte
        match reader.read_u8().await {
            Ok(byte) => {
                // Yes, there is more

                let (body, remainder, trailers) = reader.into_inner();

                // Add the byte we read
                bytes.put_u8(byte);

                // And the remainder
                bytes.put(remainder);

                Ok((body, false, bytes.into(), trailers))
            }

            Err(error) => {
                if error.kind() == io::ErrorKind::UnexpectedEof {
                    // Actually, we *do* expect EOF :)
                    let (body, _, trailers) = reader.into_inner();
                    Ok((body, true, bytes.into(), trailers))
                } else {
                    Err(error.into())
                }
            }
        }
    }
}
