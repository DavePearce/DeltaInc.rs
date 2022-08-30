/// Provides a generic notion of a sequence (which is e.g. suitable
/// for use with a lexer).  A default implementation is provided for
/// `Vec`.
use std::ops::Index;

/// Represents something corresponds to a sequence of items, such as a
/// `Vec`.
pub trait Sequence<T>: Index<usize,Output=T> {
    /// Get the length of the underlying sequence.
    fn len(&self) -> usize;
}

/// Default implementation of sequence for Vec<T>.
impl<T> Sequence<T> for Vec<T> {
    fn len(&self) -> usize { Vec::len(self) }
}
