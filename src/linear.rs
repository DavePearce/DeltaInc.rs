use std::ops::Range;
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

struct Linear<T> {
    /// Internal vector which stores concrete items.
    items: Vec<Span<T>>
}

impl<T:Iterator> From<&T> for Linear<T::Item> {
    fn from(iter: &T) -> Self {
        todo!("GOT HERE");
    }
}
