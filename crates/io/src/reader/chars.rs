use {kutil_std::iter::*, std::io, utf8_chars::*};

/// Has an [Iterator] of char.
pub trait HasCharIterator {
    /// [Iterator] of char. Will end iteration if there is an error.
    fn chars(&mut self) -> impl Iterator<Item = char>;
}

impl<BufReadT> HasCharIterator for BufReadT
where
    BufReadT: io::BufRead,
{
    fn chars(&mut self) -> impl Iterator<Item = char> {
        ConvertingIterator::new(self.chars_raw(), |result: Result<_, _>| result.ok())
    }
}
