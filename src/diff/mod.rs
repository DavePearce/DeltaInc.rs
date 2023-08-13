mod slice;
mod rewrite;
mod vec_delta;

use std::result::Result;

pub use rewrite::*;
pub use vec_delta::*;
pub use slice::*;

// ===================================================================
// Diff
// ===================================================================

/// A trait capturing the notion of a type where a _delta_ can be
/// computed between two of its items.  For example, given
/// `a1=[0,1,2]` and `a2=[1,1,2]`, we could compute a delta `d` such
/// that `a1==a2` after `a1.transform(d)`.  Observe that the delta has
/// an _orientation_ which (implicitly) means it goes from _this_ item
/// to the _other_,
pub trait Diff {
    /// Represents a delta computed by diffing two values of this
    /// type.
    type Delta;
    /// Compute a diff between this item and another, yielding a delta
    /// `d` such that `this.transform(d) == other` holds.
    fn diff(&self, other: &Self) -> Self::Delta;
}

// ===================================================================
// Transform
// ===================================================================

/// A trait describing something which can be _transformed_ in place
/// by applying a _delta_.  For example, an array `[0,1,2]` can be
/// transformed into another `[3,1,2]` by applying a delta which
/// assigns element `0` to `3`.  This trait describes the imperative
/// case where the receiver is modified in place.
pub trait Transform {
    /// Represents a delta between two values of this type.
    type Delta;
    /// Apply a given delta to this transformable item, yielded a
    /// potentially updated version of this item.
    fn transform(&mut self,d: &Self::Delta);
}

/// A trait describing something which can be _transformed_ in place
/// by applying a _delta_.  For example, an array `[0,1,2]` can be
/// transformed into another `[3,1,2]` by applying a delta which
/// assigns element `0` to `3`.  This trait describes the imperatice
/// case where the receiver is modified in place.  **Furthermore, this
/// trait allows for the possibility that an error is returned**.
pub trait TryTransform : Sized {
    /// Represents a delta between two values of this type.
    type Delta;
    /// Represents an error arising if the transform fails.
    type Error;
    /// Apply a given delta to this transformable item, yielding a
    /// potentially updated version of this item.
    fn try_transform(&mut self,d: &Self::Delta) -> Result<(),Self::Error>;
}
