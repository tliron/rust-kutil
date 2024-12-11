use std::iter::*;

//
//  IterateWithFirstLast
//

/// Iterator providing first and last flags.
pub struct IterateWithFirstLast<ItemT, IterableT>
where
    IterableT: IntoIterator<Item = ItemT>,
{
    iterator: Peekable<IterableT::IntoIter>,
    first: bool,
}

impl<ItemT, IterableT> IterateWithFirstLast<ItemT, IterableT>
where
    IterableT: IntoIterator<Item = ItemT>,
{
    /// Constructor.
    pub fn new(iterable: IterableT) -> Self {
        Self { iterator: iterable.into_iter().peekable(), first: true }
    }
}

impl<ItemT, IterableT> Iterator for IterateWithFirstLast<ItemT, IterableT>
where
    IterableT: IntoIterator<Item = ItemT>,
{
    type Item = (ItemT, bool, bool);

    fn next(&mut self) -> Option<Self::Item> {
        let first = if self.first {
            self.first = false;
            true
        } else {
            false
        };

        self.iterator.next().map(|i| (i, first, self.iterator.peek().is_none()))
    }
}

//
//  IterateWithFirst
//

/// Iterator providing first flag.
pub struct IterateWithFirst<ItemT, IterableT>
where
    IterableT: IntoIterator<Item = ItemT>,
{
    iterator: IterableT::IntoIter,
    first: bool,
}

impl<ItemT, IterableT> IterateWithFirst<ItemT, IterableT>
where
    IterableT: IntoIterator<Item = ItemT>,
{
    /// Constructor.
    pub fn new(iterable: IterableT) -> Self {
        Self { iterator: iterable.into_iter(), first: true }
    }
}

impl<ItemT, IterableT> Iterator for IterateWithFirst<ItemT, IterableT>
where
    IterableT: IntoIterator<Item = ItemT>,
{
    type Item = (ItemT, bool);

    fn next(&mut self) -> Option<Self::Item> {
        let first = if self.first {
            self.first = false;
            true
        } else {
            false
        };

        self.iterator.next().map(|i| (i, first))
    }
}

//
//  IterateWithLast
//

/// Iterator providing last flag.
pub struct IterateWithLast<ItemT, IterableT>
where
    IterableT: IntoIterator<Item = ItemT>,
{
    iterator: Peekable<IterableT::IntoIter>,
}

impl<ItemT, IterableT> IterateWithLast<ItemT, IterableT>
where
    IterableT: IntoIterator<Item = ItemT>,
{
    /// Constructor.
    pub fn new(iterable: IterableT) -> Self {
        Self { iterator: iterable.into_iter().peekable() }
    }
}

impl<ItemT, IterableT> Iterator for IterateWithLast<ItemT, IterableT>
where
    IterableT: IntoIterator<Item = ItemT>,
{
    type Item = (ItemT, bool);

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next().map(|i| (i, self.iterator.peek().is_none()))
    }
}
