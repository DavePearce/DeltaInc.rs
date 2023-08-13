use std::ops::Range;
use crate::util::Region;
use super::{SliceRewrite};

/// A `VecDelta` is a sequence of zero (or more) rewrites that can be
/// generated from something resembling a sequence (e.g. a slice or
/// `Vec`) and applied to a `Vec` to generate another `Vec`. Consider
/// the following:
///
/// ```txt
///  0 1 2 3 4 5 6 7 8 9 A B
/// +-+-+-+-+-+-+-+-+-+-+-+-+
/// |H|e|L|L|L|O| |W|o|r|l|d|
/// +-+-+-+-+-+-+-+-+-+-+-+-+
///     | : : : |    | |
///     +-------+    +-+
///         |        |
///        \|/      \|/
///     +-+-+-+-+-+-+-+
///  ...|l|l|o|...|O|R|...
///     +-+-+-+-+-+-+-+
///  0 1 2 3 4 5 6 7 8 9 A B
/// ```
///
/// Here, we have a sequence of two rewrites. Rewrites are always
/// stored in sorted order such that: **(a)** they are not adjacent
/// (i.e. as then they could be merged); and **(b)** they do not
/// overlap (i.e. as order of application is ambiguous). We also
/// assume that the starting offset for each replacement is in terms
/// of the *final* array (reading left-to-right). Thus, the above is
/// encoded internally as the sequence `(2;4;"llo"),(7;2;"OR")`.
#[derive(Clone,Debug,PartialEq)]
pub struct VecDelta<T> {
    /// Meta data describing rewrites.  For each element, the first
    /// region denotes the portion of the sequence being rewritten.
    /// In contrast, the second region denotes the subset of the
    /// `data` array being used for the rewrite.  **NOTE:** the offset
    /// of the first region is relative to the _target sequence_ rather
    /// than the _original sequence_.
    regions: Vec<(Region,Region)>,
    /// Items used within this delta
    data: Vec<T>
}

impl<T> VecDelta<T> {
    /// Construct an empty `VecDelta`
    pub const fn new() -> Self { VecDelta{regions: Vec::new(), data: Vec::new()} }

    /// Get the number of atomic rewrites represented by this delta.
    pub fn len(&self) -> usize { self.regions.len() }

    /// Check whether this delta contains any rewrites or not.
    pub fn is_empty(&self) -> bool { self.regions.is_empty() }

    /// Get the `ith` rewrite contained within this `VecDelta`.  This
    /// returns a `SliceRewrite` which refers to data held internally
    /// within this `VecDelta`.
    pub fn get(&self, ith: usize) -> Option<SliceRewrite<T>> {
        match self.regions.get(ith) {
            Some((r1,r2)) => {
                Some(SliceRewrite::new(*r1,&self.data[r2.as_range()]))
            }
            None => None
        }
    }

    /// Insert a new rewrite into this delta.  This will overwrite any
    /// existing rewrites for the given region.  This may also merge
    /// one or more existing rewrites together.  As such, after this
    /// operation, `len()` may have increased, decreased or remain the
    /// same.
    pub fn insert(&mut self, _range: Range<usize>, _data: &[T]) {
        todo!();
    }
}

impl<T:Clone> VecDelta<T> {
    /// Append a new rewrite onto the end of this delta.  This
    /// requires that rewrite logically follows all other rewrites,
    /// and is strictly disjoint from them.
    pub unsafe fn push_raw(&mut self, range: Range<usize>, data: &[T]) {
        let region : Region = range.into();
        let n = self.len();
        assert!(n == 0 || self.regions[n-1].0 < region);
        //
        let data_start = self.data.len();
        // Copy over data
        self.data.extend_from_slice(data);
        // Construct meta-data
        self.regions.push((region,Region::new(data_start,data.len())));
    }

    /// Apply this delta to a given `Vec`, thus transforming it.  This
    /// operation will `panic` if this delta is malformed with respect
    /// to the given delta.
    pub fn transform(&self, vec: &mut Vec<T>) {
        for i in 0..self.regions.len() {
            let (r1,r2) = self.regions[i];
            let data = &self.data[r2.as_range()];
            // FIXME: it would be nice to get rid of this clone
            // somehow.  In my mind, its possible to do this.
            // However, I'm not sure how to express is clearly in
            // Rust.
	    vec.splice(r1.as_range(), data.iter().cloned());
        }
    }
}

// ===================================================================
// Tests
// ===================================================================

#[cfg(test)]
mod vecdelta_tests {
    use super::{VecDelta};

    #[test]
    pub fn test_vecdelta_01() {
        let vd = VecDelta::<usize>::new();
        assert_eq!(vd.len(),0);
    }

    #[test]
    pub fn test_vecdelta_02() {
        let vd = VecDelta::<usize>::new();
        assert_eq!(vd.get(0),None);
    }

    #[test]
    pub fn test_vecdelta_03() {
        let mut vec = vec![1,2,3];
        let mut vd = VecDelta::<usize>::new();
        unsafe { vd.push_raw(0..1, &[4,5]); }
        assert_eq!(vd.len(),1);
        vd.transform(&mut vec);
        assert_eq!(vec,vec![4,5,2,3]);
    }

    #[test]
    pub fn test_vecdelta_04() {
        let mut vec = vec![1,2,3];
        let mut vd = VecDelta::<usize>::new();
        unsafe { vd.push_raw(0..1, &[4,5]); }
        unsafe { vd.push_raw(3..4, &[6,7]); }
        assert_eq!(vd.len(),2);
        vd.transform(&mut vec);
        assert_eq!(vec,vec![4,5,2,6,7]);
    }

    #[test]
    #[should_panic]
    pub fn test_vecdelta_05() {
        // Overlapping regions should cause panic!
        let mut vd = VecDelta::new();
        unsafe { vd.push_raw(0..2, &[4,5]); }
        unsafe { vd.push_raw(1..3, &[6,7]); }
    }
}
