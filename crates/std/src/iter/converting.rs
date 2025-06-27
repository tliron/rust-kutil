//
// ConvertingIterator
//

/// [Iterator] wrapper that converts its items.
#[derive(Clone, Debug)]
pub struct ConvertingIterator<ItemT, InnerIteratorT, InnerItemT> {
    iterator: InnerIteratorT,
    convert: fn(InnerItemT) -> Option<ItemT>,
}

impl<ItemT, InnerIteratorT, InnerItemT> ConvertingIterator<ItemT, InnerIteratorT, InnerItemT> {
    /// Constructor.
    pub fn new(iterator: InnerIteratorT, convert: fn(InnerItemT) -> Option<ItemT>) -> Self {
        Self { iterator, convert }
    }
}

impl<ItemT, InnerIteratorT, InnerItemT> Iterator for ConvertingIterator<ItemT, InnerIteratorT, InnerItemT>
where
    InnerIteratorT: Iterator<Item = InnerItemT>,
{
    type Item = ItemT;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next().and_then(|inner_item| (self.convert)(inner_item))
    }
}
