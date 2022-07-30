use delta_inc::{Diffable,Incremental,Transformable,Transformer};

#[derive(PartialEq,Debug,Clone,Copy)]
struct Point { x: i64, y: i64 }

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Point{x,y}
    }
}

/// A point can be transformed by applying a "delta".  That is some
/// amount of `x` and `y` that should be added to the point's `x` and
/// `y`.  Thus, for a point `p={x:1,y:2}` and a delta `d={x:1,y:1}`,
/// we would get a point `{x:2,y:3}` after applying `d` to `p`.
impl Transformable for Point {
    // The delta for a point is actually a point itself!    
    type Delta = Point;

    fn transform(&mut self,d: &Self::Delta) {
        self.x += d.x;
        self.y += d.y;
    }    
}

impl Diffable for Point {
    type Delta = Point;

    fn diff(&self, other: &Self) -> Self::Delta {
	let dx = self.x - other.x;
	let dy = self.y - other.y;
	Point{x:dx,y:dy}
    }
}

/// A simple reduction from a `Point` to a `Sum` which is done just by
/// adding the `x` and `y` fields.
impl Into<Sum> for Point {
    fn into(self) -> Sum {
        Sum(self.x + self.y)
    }
}

/// A simple example illustrating an incremental computation which
/// reduces a point to its sum `(x+y)`.
impl Incremental<Sum> for Point {
    fn update(&self, to: &Sum, delta: &Self::Delta) -> <Sum as Transformer>::Delta {
        // In this case, we can calculate the Sum delta purely from
        // the Sum delta.
        delta.x + delta.y
    }
}

/// Defines a wrapper for `i64` since we cannot implement the
/// `Transformer` trait on a type we don't own (i.e. since we don't
/// own `Transformer` here either).
#[derive(Clone,Copy,Debug,PartialEq)]
struct Sum(i64);

impl Transformer for Sum {
    type Delta = i64;

    fn transform_into(&self,d: &Self::Delta) -> Self {
        Sum(self.0 + d)
    }    
}

// ===============================================================
// Tests
// ===============================================================

#[test]
fn test_transformer_01() {
    let p = Point::new(1,2);
    let q = p.transform_into(&Point::new(1,1));
    assert_eq!(q,Point{x:2,y:3});
}

#[test]
fn test_transformable_01() {
    let mut p = Point::new(1,2);
    p.transform(&Point::new(1,1));
    assert_eq!(p,Point{x:2,y:3});
}

fn test_diff_01() {
    let p1 = Point::new(1,2);
    let p2 = Point::new(4,6);
    let d1 = p1.diff(&p2);
    let d2 = p2.diff(&p2);
    //
    assert_eq!(d1,Point::new(3,4));
    assert_eq!(d2,Point::new(-3,-4));
    //
    assert_eq!(p2,p1.transform_into(&d1));
    assert_eq!(p1,p2.transform_into(&d2));
}

#[test]
fn test_incremental_01() {
    let p = Point::new(1,2);
    let d = p.update(&Sum(3),&Point::new(1,1));
    assert_eq!(d,2);
}

#[test]
fn test_incremental_02() {
    let p1 = Point::new(2,3);
    let s1 : Sum = p1.into();
    //
    let d1 = Point::new(1,2);
    //
    let p2 = p1.transform_into(&d1);
    let s2 : Sum = p2.into();
    //
    let d2 = p1.update(&s1,&d1);
    let s3 = s1.transform_into(&d2);
    assert_eq!(s2,s3);
}
