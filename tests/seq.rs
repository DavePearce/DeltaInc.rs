use std::ops::Index;
use delta_inc::seq::{Sequence};

// ============================================================================
// Vec
// ============================================================================

#[test]
fn test_vec_01() {
    let v = vec![0,1,2];
    let s : &dyn Sequence<u32> = &v;
    assert!(s[0] == 0);
    assert!(s[1] == 1);
    assert!(s[2] == 2);
    assert!(s.len() == 3);
}

// ============================================================================
// Fixed Array
// ============================================================================

struct FixedArray<T, const N: usize> { items: [T;N] }

impl<T:Copy, const N: usize> FixedArray<T,N> {
    pub fn new(val:T) -> Self {
        FixedArray{items: [val;N]}
    }
}

impl<T,const N: usize> Index<usize> for FixedArray<T,N> {
    // Bind Index::Output to T
    type Output = T;
    //
    fn index(&self, index: usize) -> &T {
        &self.items[index]
    }
}

impl<T,const N: usize> Sequence<T> for FixedArray<T,N> {
    fn len(&self) -> usize { N }
}

#[test]
fn test_array_01() {
    let arr = FixedArray::<u32,8>::new(123);
    assert!(arr[0] == 123);
    assert!(arr.len() == 8);
}

#[test]
#[should_panic]
fn test_array_02() {
    let arr = FixedArray::<u32,8>::new(123);
    assert!(arr[8] == 123);
}
