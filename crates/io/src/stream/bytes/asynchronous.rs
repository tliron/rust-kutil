use {
    bytes::*,
    futures::{Stream, StreamExt},
    kutil_std::error::*,
    std::{cmp::*, io, pin::*, result::Result, task::*},
    tokio::io::*,
};

const REMAINDER_INITIAL_CAPACITY: usize = 8 * 1_024; // 8 KiB

//
// AsyncBytesStreamReader
//

/// A Tokio [AsyncRead] implementation for a [Stream] of [Result]\<[Bytes], _\>.
///
/// Errors are wrapped as [io::ErrorKind::Other].
///
/// Useful, for example, for reading from
/// [reqwest::Response::byte_stream](https://github.com/seanmonstar/reqwest).
pub struct AsyncBytesStreamReader<StreamT, ErrorT>
where
    StreamT: Stream<Item = Result<Bytes, ErrorT>> + Unpin,
{
    stream: StreamT,

    /// Remainder.
    pub remainder: BytesMut,
}

impl<StreamT, ErrorT> AsyncBytesStreamReader<StreamT, ErrorT>
where
    StreamT: Stream<Item = Result<Bytes, ErrorT>> + Unpin,
    ErrorT: Into<CapturedError>,
{
    /// Constructor.
    pub fn new(stream: StreamT) -> Self {
        Self { stream, remainder: BytesMut::with_capacity(0) }
    }

    /// Back to the inner [Stream].
    ///
    /// Note that the stream may have changed if we have read from this reader, in which case the
    /// returned remainder will be non-empty.
    pub fn into_inner(self) -> (StreamT, BytesMut) {
        (self.stream, self.remainder)
    }

    fn validate_remainder_capacity(&mut self) {
        let capacity = self.remainder.capacity();
        if capacity < REMAINDER_INITIAL_CAPACITY {
            self.remainder.reserve(REMAINDER_INITIAL_CAPACITY - capacity);
        }
    }

    // fn ensure_chunk(&mut self, context: &mut Context<'_>) -> Poll<io::Result<()>> {
    //     if let Some(chunk) = &self.chunk {
    //         // Are we at the end of the chunk?
    //         if self.chunk_start >= chunk.len() {
    //             self.chunk = None;
    //             self.chunk_start = 0;
    //         }
    //     }

    //     if self.chunk.is_none() {
    //         // Next chunk
    //         match self.stream.poll_next_unpin(context) {
    //             Poll::Ready(ready) => match ready {
    //                 Some(result) => match result {
    //                     Ok(chunk) => {
    //                         self.chunk = Some(chunk);
    //                     }

    //                     Err(error) => return Poll::Ready(Err(io::Error::other(error))),
    //                 },

    //                 None => {}
    //             },

    //             Poll::Pending => return Poll::Pending,
    //         }
    //     }

    //     Poll::Ready(Ok(()))
    // }
}

impl<StreamT, ErrorT> AsyncRead for AsyncBytesStreamReader<StreamT, ErrorT>
where
    StreamT: Stream<Item = Result<Bytes, ErrorT>> + Unpin,
    ErrorT: Into<CapturedError>,
{
    fn poll_read(
        mut self: Pin<&mut Self>,
        context: &mut Context<'_>,
        buffer: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        // Copy as much as we can from the remainder
        if self.remainder.has_remaining() {
            let size = min(buffer.remaining_mut(), self.remainder.remaining());

            if size != 0 {
                let bytes = self.remainder.copy_to_bytes(size);
                buffer.put(bytes);

                if !buffer.has_remaining_mut() {
                    // Buffer is full
                    return Poll::Ready(Ok(()));
                }
            }
        }

        Poll::Ready(match ready!(self.stream.poll_next_unpin(context)) {
            Some(result) => {
                match result {
                    Ok(mut bytes) => {
                        // Copy as much as we can from the bytes
                        let size = min(buffer.remaining_mut(), bytes.remaining());

                        if size != 0 {
                            let bytes = bytes.copy_to_bytes(size);
                            buffer.put(bytes);
                        }

                        // Store leftover bytes in the remainder
                        if bytes.has_remaining() {
                            self.validate_remainder_capacity();
                            self.remainder.put(bytes);
                        }

                        Ok(())
                    }

                    Err(error) => Err(io::Error::other(error)),
                }
            }

            None => Ok(()),
        })
    }
}
