use delta_inc::diff::{Diff,Transform};

#[derive(PartialEq,Debug,Clone,Copy)]
struct Point { x: i64, y: i64 }

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Point{x,y}
    }
}

// ===============================================================
// Transformer
// ===============================================================

/// A point can be transformed by applying a "delta".  That is some
/// amount of `x` and `y` that should be added to the point's `x` and
/// `y`.  Thus, for a point `p={x:1,y:2}` and a delta `d={x:1,y:1}`,
/// we would get a point `{x:2,y:3}` after applying `d` to `p`.
impl Transform for Point {
    // The delta for a point is actually a point itself!
    type Delta = Point;

    fn transform(&mut self,d: &Self::Delta) {
        self.x += d.x;
        self.y += d.y;
    }
}

/// A delta can easily be computed between two points by computing the
/// difference between them.
impl Diff for Point {
    type Delta = Point;

    fn diff(&self, other: &Self) -> Self::Delta {
	let dx = other.x - self.x;
	let dy = other.y - self.y;
	Point{x:dx,y:dy}
    }
}

// ===============================================================
// Tests
// ===============================================================

#[test]
fn test_transform_01() {
    let mut p = Point::new(1,2);
    p.transform(&Point::new(1,1));
    assert_eq!(p,Point{x:2,y:3});
}

#[test]
fn test_transform_02() {
    let mut p = Point::new(1,2);
    p.transform(&Point::new(3,4));
    assert_eq!(p,Point{x:4,y:6});
}

#[test]
fn test_diff_01() {
    let mut p1 = Point::new(1,2);
    let mut p2 = Point::new(4,6);
    let d1 = p1.diff(&p2);
    let d2 = p2.diff(&p1);
    //
    assert_eq!(d1,Point::new(3,4));
    assert_eq!(d2,Point::new(-3,-4));
    //
    p1.transform(&d1);
    p2.transform(&d2);
    //
    assert_eq!(Point::new(4,6),p1);
    assert_eq!(Point::new(1,2),p2);
}
