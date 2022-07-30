/// Provides default implementations of the `Transformable` trait.
use std::ops::Range;
use crate::{Transformable};
use crate::region::Region;

struct VecRewrite<T> {
    /// Portion of `Vec<T>` being replaced.
    region: Region,
    /// Data being used for replacement
    data: Vec<T>
}

impl<T> VecRewrite<T> {
    pub fn new(region: Region, data: Vec<T>) -> Self {
	VecRewrite{region,data: data}
    }
}

pub struct VecDelta<T> {
    /// List of patches in sorted order.
    rewrites: Vec<VecRewrite<T>>
}

impl<T> VecDelta<T> {
    pub fn replace(range: Range<usize>, data: Vec<T>) -> Self {
	let rw = VecRewrite::new(range.into(),data);
	VecDelta{rewrites: vec![rw]}
    }    
    pub fn and_replace(&mut self, range: Range<usize>, data: Vec<T>) {
    	self.rewrites.push(VecRewrite::new(range.into(),data));
    }
}

impl<T:Default + std::clone::Clone> Transformable for Vec<T> {
    type Delta = VecDelta<T>;
    
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

