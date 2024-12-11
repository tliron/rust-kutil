use {futures::*, std::ops::*, tokio::runtime};

//
// BlockingStream
//

/// Blocking stream for Tokio.
pub struct BlockingStream<StreamT>
where
    StreamT: Stream + Unpin,
{
    stream: StreamT,
    runtime: runtime::Runtime,
}

impl<StreamT> BlockingStream<StreamT>
where
    StreamT: Stream + Unpin,
{
    /// Constructor.
    pub fn new(stream: StreamT, runtime: runtime::Runtime) -> Self {
        Self { stream, runtime }
    }

    /// Into inner.
    pub fn into_inner(self) -> StreamT {
        self.stream
    }
}

impl<StreamT> Deref for BlockingStream<StreamT>
where
    StreamT: Stream + Unpin,
{
    type Target = StreamT;

    fn deref(&self) -> &Self::Target {
        &self.stream
    }
}

impl<StreamT> DerefMut for BlockingStream<StreamT>
where
    StreamT: Stream + Unpin,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.stream
    }
}

impl<StreamT: Stream + Unpin> Iterator for BlockingStream<StreamT> {
    type Item = StreamT::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.runtime.block_on(self.stream.next())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.stream.size_hint()
    }
}
