use delta_inc::Transformable;
use delta_inc::vec;

// ===============================================================
// Replace
// ===============================================================

#[test]
fn test_replace_01() {
    // Check unit replacement.
    let mut v1 = vec![0,2,3];
    // Construct delta
    let d = vec::replace(0..1,vec![1]);
    // Apply delta
    v1.transform(&d);
    // Check outcome!
    assert_eq!(vec![1,2,3],v1);
}

#[test]
fn test_replace_02() {
    // Check multi-replacement
    let mut v1 = vec![1,2,3];
    // Construct delta
    let d = vec::replace(0..2,vec![1,0]);
    // Apply delta
    v1.transform(&d);
    // Check outcome!
    assert_eq!(vec![1,0,3],v1);
}

#[test]
fn test_replace_03() {
    // Check negative sized replacement
    let mut v1 = vec![1,2,3];
    // Construct delta
    let d = vec::replace(0..2,vec![0]);
    // Apply delta
    v1.transform(&d);
    // Check outcome!
    assert_eq!(vec![0,3],v1);
}

#[test]
fn test_replace_04() {
    // Check multi replacement
    let mut v1 = vec![1,2,3];
    // Construct delta
    let mut d = vec::replace(0..0,vec![0]);
    // Append replacement
    d.and_replace(1..3,vec![4,5,6]);
    // Apply delta
    v1.transform(&d);
    // Check outcome!
    assert_eq!(vec![0,4,5,6,3],v1);
}

// ======================================================
// Insert
// ======================================================

#[test]
fn test_insert_01() {
    // Check insertion.
    let mut v1 = vec![1,2,3];
    // Construct delta
    let d = vec::insert(0,vec![0]);
    // Apply delta
    v1.transform(&d);
    // Check outcome!
    assert_eq!(vec![0,1,2,3],v1);
}

// ======================================================
// Remove
// ======================================================

#[test]
fn test_remove_01() {
    // Check negative sized replacement
    let mut v1 = vec![1,2,3];
    // Construct delta
    let d = vec::remove(0..2);
    // Apply delta
    v1.transform(&d);
    // Check outcome!
    assert_eq!(vec![3],v1);
}
