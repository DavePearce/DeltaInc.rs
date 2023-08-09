/// Provides default implementations of the `Transform` trait.
use std::ops::Range;
use crate::diff::Transform;
use crate::util::Region;

// ===================================================================
// Rewrite
// ===================================================================

/// An atomic action applied to a `Vec<T>`, such as replace one region
/// by another or inserting one or more items, etc.
#[derive(Clone)]
pub struct Rewrite<T> {
    /// Portion of `Vec<T>` being replaced.
    region: Region,
    /// Data being used for replacement
    data: Vec<T>
}

impl<T> Rewrite<T> {
    pub fn new(region: Region, data: Vec<T>) -> Self {
	Rewrite{region,data: data}
    }

    /// Map this rewrite to another rewrite of a different type.  This
    /// is useful when applying rewrites to structures which include
    /// meta-data.
    pub fn map<S,F>(&self, func: F) -> Rewrite<S>
    where F:FnMut(&T)->S {
        let data : Vec<S> = self.data.iter().map(func).collect();
        Rewrite{region:self.region,data}
    }
}

// ===================================================================
// Delta
// ===================================================================

/// A set of zero or more rewrites which can be applied atomically to
/// transform a vector.
pub struct Delta<T> {
    /// List of patches in sorted order.
    rewrites: Vec<Rewrite<T>>
}

impl<T> Delta<T> {
    pub fn and_replace(&mut self, range: Range<usize>, data: Vec<T>) {
    	self.rewrites.push(Rewrite::new(range.into(),data));
    }

    /// Construct an iterator over the rewrites contained within this
    /// delta.
    pub fn iter(&self) -> std::slice::Iter<Rewrite<T>> {
        self.rewrites.iter()
    }
}

/// Construct a Delta from a slice of rewrites.
impl<T:Clone> From<&[Rewrite<T>]> for Delta<T> {
    fn from(rewrites: &[Rewrite<T>]) -> Delta<T> {
        Delta{rewrites: rewrites.to_vec()}
    }
}

/// Construct a Delta from a vector of rewrites.
impl<T> From<Vec<Rewrite<T>>> for Delta<T> {
    fn from(rewrites: Vec<Rewrite<T>>) -> Delta<T> {
        Delta{rewrites}
    }
}

/// Constract a delta which inserts a given range of elements at a
/// given point in the vector.
pub fn insert<T>(index: usize, data: Vec<T>) -> Delta<T> {
    let rw = Rewrite::new(Region::new(index,0),data);
    Delta{rewrites: vec![rw]}
}

/// Construct a delta which replaces a given range of elements with
/// another sequence of zero or more items.
pub fn replace<T>(range: Range<usize>, data: Vec<T>) -> Delta<T> {
    let rw = Rewrite::new(range.into(),data);
    Delta{rewrites: vec![rw]}
}

/// Construct a delta which removes one or more elements from the
/// vector.
pub fn remove<T>(range: Range<usize>) -> Delta<T> {
    let rw = Rewrite::new(range.into(),Vec::new());
    Delta{rewrites: vec![rw]}
}

// ===================================================================
// Transform
// ===================================================================

impl<T:std::clone::Clone> Transform for Vec<T> {
    type Delta = Delta<T>;

    fn transform(&mut self,d: &Self::Delta) {
	// NOTE: this is a very inefficient implementation which I
	// have written as scafolding to get this library up and
	// running.
	for rw in &d.rewrites {
	    // Apply rewrite.
	    self.splice(rw.region.as_range(), rw.data.iter().cloned());
	}
    }
}
