/// A trait describing something which can be _transformed_ by
/// applying a _delta_.  For example, an array `[0,1,2]` can be
/// transformed into another `[3,1,2]` by applying a delta which
/// assigns element `0` to `3`.  **This trait describes the functional
/// case, where the receiver is not modified**.
pub trait Transformer {
    type Delta;

    /// Apply a given delta to this transformable item, yielded a
    /// potentially updated version of this item.
    fn transform_into(&self,d: &Self::Delta) -> Self;
}

/// A trait describing something which can be _transformed_ in place
/// by applying a _delta_.  For example, an array `[0,1,2]` can be
/// transformed into another `[3,1,2]` by applying a delta which
/// assigns element `0` to `3`.  **In this case, the receiver is
/// modified in place**.
pub trait Transformable {
    type Delta;

    /// Apply a given delta to this transformable item, yielded a
    /// potentially updated version of this item.
    fn transform(&mut self,d: &Self::Delta);
}

/// Provide a default trait implementation for every type which is
/// transformable.  This first clones the item, and then transforms
/// it in place.
impl<T: Transformable + Clone> Transformer for T {
    type Delta = T::Delta;

    fn transform_into(&self,d: &Self::Delta) -> Self {
        // Clone
        let mut r = self.clone();
        // Transform
        r.transform(d);
        // Done
        r
    }
}

/// A trait capturing the essence of an incremental computation from
/// `self` to some item `T`.  This is similar to the `Into` trait, but
/// with the ability to work with _deltas_.  To understand this,
/// consider the following:
///
/// ```Rust
/// let t1 : T = self.into()
/// //
/// self.transform(&d1);
/// //
/// let t2 : T = self.into()
/// ```
///
/// This assumes some delta `d` which can be applied to `self`.  The
/// issue with this is that the final transformation to `t2` is
/// _computed from scratch_.  This could be expensive, though it might
/// be necessary.  However, for some computations, however, we can
/// reduce the amount of work done by _incrementally_ applying `into`
/// through the delta `d`.  That would look like this:
///
/// ```Rust
/// let t1 : T = self.into()
/// //
/// let d2 = self.update(&t1,&d1);
/// //
/// let t2 = t1.transform_into(&d2);
/// ```
///
/// This update version produces the same `t2` as the original, but
/// allows for the computation to exploit existing information about
/// `t1`.  In some cases, this can make a big difference.  For
/// example, consider parsing algorithm which turns a _source file_
/// into an _Abstract Syntax Tree_.  In this case, a delta is some
/// change to the source file.  Then, given the AST for a source file
/// and a change to that source file, the resulting AST might be
/// almost entirely (or actually) the same as the original file.  For
/// example, if the change removed a line of whitespace then nothing
/// changes.
pub trait Incremental<T:Transformer> : Transformable + Into<T> {
   fn update(&self, to: &T, delta: &Self::Delta) -> T::Delta;
}

