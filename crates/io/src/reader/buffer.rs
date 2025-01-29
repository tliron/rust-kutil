use std::{io::*, sync::*};

// Why do we need this?
//
// Because io::Cursor relies on AsRef<[u8]>, but Arc<Vec<u8>> does not implement it.

/// An immutable buffer that can be read concurrently.
#[derive(Clone, Debug)]
pub struct ReadableBuffer(Arc<Vec<u8>>);

/// [ReadableBuffer] reader.
pub type ReadableBufferReader = Cursor<ReadableBuffer>;

impl ReadableBuffer {
    /// Constructor.
    pub fn new(buffer: Vec<u8>) -> Self {
        Self(Arc::new(buffer))
    }

    /// Reader.
    pub fn reader(&self) -> ReadableBufferReader {
        // We are cloning the Arc, not the buffer
        ReadableBufferReader::new(self.clone())
    }
}

impl AsRef<[u8]> for ReadableBuffer {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
