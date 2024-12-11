use super::super::stream::*;

use {
    bytes::*,
    futures::*,
    std::{error::*, io},
};

//
// BlockingBytesStreamReader
//

/// A [Read](io::Read) implementation for a [Stream] of [Result]\<[Bytes], _\>.
///
/// Errors are wrapped as [io::ErrorKind::Other].
///
/// Useful, for example, for reading from
/// [reqwest::Response::byte_stream](https://github.com/seanmonstar/reqwest).
pub struct BlockingBytesStreamReader<StreamT, ErrorT>
where
    StreamT: Stream<Item = Result<Bytes, ErrorT>> + Unpin,
{
    stream: BlockingStream<StreamT>,
    chunk: Option<Bytes>,
    chunk_start: usize,
}

impl<StreamT, ErrorT> BlockingBytesStreamReader<StreamT, ErrorT>
where
    StreamT: Stream<Item = Result<Bytes, ErrorT>> + Unpin,
    ErrorT: Into<Box<dyn Error + Send + Sync>>,
{
    /// Constructor.
    pub fn new(stream: BlockingStream<StreamT>) -> Self {
        Self { stream, chunk: None, chunk_start: 0 }
    }

    fn ensure_chunk(&mut self) -> io::Result<()> {
        if let Some(chunk) = &self.chunk {
            // Are we at the end of the chunk?
            if self.chunk_start >= chunk.len() {
                self.chunk = None;
                self.chunk_start = 0;
            }
        }

        if self.chunk.is_none() {
            // Next chunk
            self.chunk = self.stream.next().transpose().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        }

        Ok(())
    }
}

impl<StreamT, ErrorT> io::Read for BlockingBytesStreamReader<StreamT, ErrorT>
where
    StreamT: Stream<Item = Result<Bytes, ErrorT>> + Unpin,
    ErrorT: Into<Box<dyn Error + Send + Sync>>,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let buf_len = buf.len();
        let mut buf_start = 0;
        let mut buf_end = 0;

        while buf_end < buf_len {
            self.ensure_chunk()?;

            match &self.chunk {
                Some(chunk) => {
                    // What we want
                    let mut chunk_end = self.chunk_start + (buf_len - buf_start);

                    // What we can do
                    let chunk_len = chunk.len();
                    if chunk_end > chunk_len {
                        chunk_end = chunk_len;
                        buf_end = buf_start + (chunk_end - self.chunk_start);
                    } else {
                        buf_end = buf_len;
                    }

                    buf[buf_start..buf_end].copy_from_slice(&chunk[self.chunk_start..chunk_end]);

                    buf_start = buf_end;
                    self.chunk_start = chunk_end;
                }

                None => break,
            }
        }

        Ok(buf_end)
    }
}
