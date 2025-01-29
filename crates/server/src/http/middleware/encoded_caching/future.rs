/// Encoding future.
pub struct EncodingFuture {}

impl EncodingFuture {
    /// Constructor.
    pub fn new() -> Self {
        EncodingFuture {}
    }
}

impl Future for EncodingFuture {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        Ok(Async::Ready(()))
    }
}

impl Future for EncodingFuture {}
