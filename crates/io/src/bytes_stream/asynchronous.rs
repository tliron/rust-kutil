use {
    bytes::*,
    futures::*,
    std::{error::*, io, pin::*, task::*},
    tokio::io::{AsyncRead, ReadBuf},
};

//
// AsyncBytesStreamReader
//

/// An [AsyncRead] implementation for a [Stream] of [Result]\<[Bytes], _\>.
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
    chunk: Option<Bytes>,
    chunk_start: usize,
}

impl<StreamT, ErrorT> AsyncBytesStreamReader<StreamT, ErrorT>
where
    StreamT: Stream<Item = Result<Bytes, ErrorT>> + Unpin,
    ErrorT: Into<Box<dyn Error + Send + Sync>>,
{
    /// Constructor.
    pub fn new(stream: StreamT) -> Self {
        Self { stream, chunk: None, chunk_start: 0 }
    }

    fn ensure_chunk(&mut self, context: &mut Context<'_>) -> Poll<io::Result<()>> {
        if let Some(chunk) = &self.chunk {
            // Are we at the end of the chunk?
            if self.chunk_start >= chunk.len() {
                self.chunk = None;
                self.chunk_start = 0;
            }
        }

        if self.chunk.is_none() {
            // Next chunk
            match self.stream.poll_next_unpin(context) {
                Poll::Ready(ready) => match ready {
                    Some(result) => match result {
                        Ok(chunk) => {
                            self.chunk = Some(chunk);
                        }

                        Err(error) => return Poll::Ready(Err(io::Error::new(io::ErrorKind::Other, error))),
                    },

                    None => {}
                },

                Poll::Pending => return Poll::Pending,
            }
        }

        Poll::Ready(Ok(()))
    }
}

impl<StreamT, ErrorT> AsyncRead for AsyncBytesStreamReader<StreamT, ErrorT>
where
    StreamT: Stream<Item = Result<Bytes, ErrorT>> + Unpin,
    ErrorT: Into<Box<dyn Error + Send + Sync>>,
{
    fn poll_read(self: Pin<&mut Self>, context: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<io::Result<()>> {
        let this = self.get_mut();

        let status = this.ensure_chunk(context);

        if let Poll::Ready(ready) = &status {
            if ready.is_ok() {
                if let Some(chunk) = &this.chunk {
                    // What we want
                    let mut chunk_end = this.chunk_start + buf.remaining();

                    // What we can do
                    let chunk_len = chunk.len();
                    if chunk_end > chunk_len {
                        chunk_end = chunk_len;
                    }

                    buf.put_slice(&chunk[this.chunk_start..chunk_end]);

                    this.chunk_start = chunk_end;
                }
            }
        }

        status
    }
}
