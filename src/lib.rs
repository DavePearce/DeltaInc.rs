/// A trait describing something which can be _transformed_ by
/// applying a _delta_.  For example, an array `[0,1,2]` can be
/// transformed into another `[3,1,2]` by applying a delta which
/// assigns element `0` to `3`.
trait Transformable {
    type Delta;

    /// Apply a given delta to this transformable item, yielded a
    /// potentially updated version of this item.
    fn transform(&self,d: Self::Delta) -> Self;
}

/// A trait capturing the essence of an incremental computation.
trait Incremental<T:Transformable> : Transformable {
   fn update(&self, to: T, delta: Self::Delta) -> T::Delta;
}

