//
// Parseable
//

/// Parseable.
pub trait Parseable: Sized {
    /// Parse.
    fn parse(representation: &str) -> Option<Self>;
}
