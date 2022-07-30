use delta_inc::{Transformable,VecDelta};

// ===============================================================
// Tests
// ===============================================================

#[test]
fn test_vec_01() {
    // Check unit replacement.
    let mut v1 = vec![0,2,3];
    // Construct delta
    let mut d = VecDelta::replace(0..1,vec![1]);
    // Apply delta
    v1.transform(&d);
    // Check outcome!
    assert_eq!(vec![1,2,3],v1);
}

#[test]
fn test_vec_02() {
    // Check insertion.
    let mut v1 = vec![1,2,3];
    // Construct delta
    let mut d = VecDelta::replace(0..0,vec![0]);
    // Apply delta
    v1.transform(&d);
    // Check outcome!
    assert_eq!(vec![0,1,2,3],v1);
}

#[test]
fn test_vec_03() {
    // Check multi-replacement
    let mut v1 = vec![1,2,3];
    // Construct delta
    let mut d = VecDelta::replace(0..2,vec![1,0]);
    // Apply delta
    v1.transform(&d);
    // Check outcome!
    assert_eq!(vec![1,0,3],v1);
}

#[test]
fn test_vec_04() {
    // Check deletion
    let mut v1 = vec![1,2,3];
    // Construct delta
    let mut d = VecDelta::replace(0..2,vec![0]);
    // Apply delta
    v1.transform(&d);
    // Check outcome!
    assert_eq!(vec![0,3],v1);
}

#[test]
fn test_vec_05() {
    // Check multi replacement
    let mut v1 = vec![1,2,3];
    // Construct delta
    let mut d = VecDelta::replace(0..0,vec![0]);
    // Append replacement
    d.and_replace(1..3,vec![4,5,6]);
    // Apply delta
    v1.transform(&d);
    // Check outcome!
    assert_eq!(vec![0,4,5,6,3],v1);
}

