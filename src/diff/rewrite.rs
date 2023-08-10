use std::marker::PhantomData;
use crate::util::Region;

/// Describes an _atomic rewrite_ of some source array (slice, `Vec`,
/// etc). Specifically, a region in the source array is replaced by a
/// new sequence. Observe that this region may be larger or smaller
/// than the original region. The following illustrates:
///
/// ```txt
///  0 1 2 3 4 5 6 7 8 9 A B
/// +-+-+-+-+-+-+-+-+-+-+-+-+
/// |H|e|L|L|L|O| |W|o|r|l|d|
/// +-+-+-+-+-+-+-+-+-+-+-+-+
///     | : : : |
///     +-------+
///         |
///        \|/
///      +-+-+-+
///      |l|l|o|
///      +-+-+-+
/// ```
///
/// The above illustrates the region "LLLO" from the original sequence
/// being replaced by "llo". Thus, after applying the rewrite, we have
/// "Hello World". This replacement is denoted as the triple
/// `(2;4;"llo")` which indicates the replacement begins at position
/// `2`, replaces `4` items from the original array with a given
/// sequence of zero or more items.
#[derive(Clone,Debug)]
pub struct Rewrite<S,T:AsRef<[S]>> {
    /// Portion of `Vec<T>` being replaced.
    region: Region,
    /// Data being used for replacement
    data: T,
    // dummy field
    dummy: PhantomData<S>
}

impl<S,T:AsRef<[S]>> Rewrite<S,T> {
    pub fn new(region: Region, data: T) -> Self {
        let dummy = PhantomData;
	Self{region,data,dummy}
    }
}

impl<S,T:AsRef<[S]>+PartialEq> PartialEq for Rewrite<S,T> {
    fn eq(&self, other: &Self) -> bool {
        self.region == other.region && self.data == other.data
    }
}

// ===================================================================
// Common Aliases
// ===================================================================

/// A rewrite which contains its data internally as a `Vec<T>`.
pub type VecRewrite<T> = Rewrite<T,Vec<T>>;

/// A rewrite which contains its data internally as a slice `&[T]`.
/// This is useful when rewrites are encoded within an external data
/// structure (e.g. `VecDelta`) and we want temporary access to them
/// (e.g. to iterate over them).
pub type SliceRewrite<'a,T> = Rewrite<T,&'a [T]>;

// ===================================================================
// Tests
// ===================================================================

#[cfg(test)]
mod tests {
    use crate::diff::rewrite::*;

    #[test]
    fn test_vec_01() {
        let items = vec![1,2,3];
        let rw = Rewrite::new(Region::new(0,1), items);
        assert_eq!(rw.region.offset,0);	
    }

    #[test]
    fn test_slice_01() {
        let items = vec![1,2,3];
        let rw = Rewrite::new(Region::new(0,1), &items);
        assert_eq!(rw.region.offset,0);
    }
}
