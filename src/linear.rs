use std::ops::{Index,Range};
// This is really just a "sketch for now"

/// Associates a given item with linear region of some sequence.
#[derive(Debug)]
pub struct Span<T> {
    /// The item being associated by this span.
    pub item: T,
    /// Identifies the (half open) region in the sequence.
    pub region: Range<usize>
}

// ==================================================================
// Linear(isation)
// ==================================================================

/// A data structure which provides a linearisation of an _underlying
/// sequence_.  For example, we can divide a sequence
/// (e.g. characters) up into non-overlapping segments (e.g. lines).
pub struct Linear<T> {
    /// Internal vector which stores concrete items.
    items: Vec<Span<T>>
}

impl<T> Linear<T> {
    pub fn len(&self) -> usize { self.items.len() }

    /// Get the span at a given position in this linearisation.
    pub fn get(&self, index: usize) -> &Span<T> {
        &self.items[index]
    }

    /// Get the span enclosing a given index in the underlying
    /// sequence (or none if nothing encloses that position).
    pub fn get_enclosing(&self, index: usize) -> Option<&Span<T>> {
        // Binary search, basically.
        todo!["implement me"]
    }
}

/// Allow a linear value to be constructed from an iterator.
impl<S,T:Iterator<Item=Span<S>>> From<T> for Linear<S> {
    fn from(iter: T) -> Self {
        Linear{items: iter.collect()}
    }
}

/// Overload the [] operator for linear.
impl<T> Index<usize> for Linear<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index].item
    }
}
