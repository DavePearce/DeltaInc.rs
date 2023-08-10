use super::{Diff,VecDelta};

/// An implementation of the `Diff` trait for arbritrary slices.  This
/// is implemented using the well-known _longest common subsequence_
/// algorithm.
impl<T:Clone+PartialEq> Diff for [T] {
    type Delta = VecDelta<T>;

    fn diff(&self, other: &[T]) -> Self::Delta {
        // FIXME: reduce number of allocations!
        let mapping = longest_common_subsequence(self,other);
        // Convert mapping to rewrites
        extract_delta(&mapping, other)
    }
}

/// Determine the longest common subsequence of two slices. For
/// example, suppose `lhs=[a,b,b,c,b,c,d]` and `rhs=[b,b,e,c,d,e]` then a
/// *common subsequence* is `[b,b]` and another is `[b,c,d]`. However,
/// the *longest common subsequence* is `[b,b,c,d]`.
///
/// This algorithm produces a mapping from elements in `lhs` to
/// elements in rhs. For the above example, it might produce
/// `[_,0,1,3,_,_,4]`. Here, `_` indicates the element in `lhs` does
/// not match an element in `rhs`. Otherwise, values are the matching
/// indices in `rhs`.
///
/// **NOTE:** Whilst the above example had only one longest common
/// subsequence, it's not always the case there is only one. This
/// algorithm simply returns *a* longest common subsequence.
///
/// # References
///
/// * _Introduction to Algorithms_, T.H Cormen, C.E. Leiserson,
/// R.L. Rivert and C. Stein, 2nd ed.  Chapter 15.
pub fn longest_common_subsequence<T:Clone+PartialEq>(lhs: &[T], rhs: &[T]) -> Vec<Option<usize>> {
    let m = lhs.len() + 1;
    let n = rhs.len() + 1;
    let mut c = vec![0; m * n];
    // Calculate the lengths
    for i in 0 .. lhs.len() {
        let ip1 = i+1;
        for j in 0 .. rhs.len() {
            let jp1 = j+1;
            let ij = ip1 + (jp1 * m);
            if lhs[i] == rhs[j] {
                c[ij] = c[i + (j * m)] + 1;
            } else {
                let c_ijp1 = c[i + (jp1 * m)];
                let c_ip1j = c[ip1 + (j * m)];
                c[ij] = if c_ijp1 >= c_ip1j { c_ijp1 } else { c_ip1j };
            }
        }
    }
    // Finally, extract the LCS
    let mut res = vec![None;lhs.len()];
    extract_subsequence(&c, &mut res, m - 1, n - 1);
    res
}

fn extract_subsequence<T:PartialEq>(c: &[T], res: &mut [Option<usize>], i: usize, j: usize) {
    let m = res.len() + 1;
    if i > 0 && j > 0 {
        let c_ij = &c[i + (j * m)];
        let c_im1j = &c[(i - 1) + (j * m)];
        let c_ijm1 = &c[i + ((j - 1) * m)];
        if c_ij == c_im1j {
            res[i - 1] = None;
            extract_subsequence(c, res, i - 1, j);
        } else if c_ij == c_ijm1 {
            res[i - 1] = None;
            extract_subsequence(c, res, i, j - 1);
        } else {
            extract_subsequence(c, res, i - 1, j - 1);
            res[i - 1] = Some(j - 1);
        }
    }
}

/// Extract delta's using a given mapping from the before sequence to
/// the after sequence, as generated from the _least common
/// subsequence_ algorithm.  For example:
///
/// ```txt
///  0 1 2
/// +-+-+-+-+-+
/// |a|b|c|d|e| (before)
/// +-+-+-+-+-+
///    | |
///   / /
///  | |
/// +-+-+-+-+
/// |b|c|f|g| (after)
/// +-+-+-+-+
/// ```
///
/// In this case, the mapping would be `[-1,0,1,-1,-1]` which
/// indicates positions `0`, `3` and `4` are removed whilst positions
/// `1` and `2` correspond to positions `0` and `1` in the final
/// sequence.
///
/// The current extraction mechanism could still be improved in that
/// it can generate lots of small delta's when a single large one
/// would be more sensible. Potentially, some form of post processing
/// could coalesce delta's as necessary.
fn extract_delta<T:Clone>(mapping: &[Option<usize>], after: &[T]) -> VecDelta<T> {
    let mut delta = VecDelta::new();
    println!("MAPPING: {mapping:?}");
    // Initialise after markers
    let (mut a_start, mut a_pos) = (0,0);
    // Initialise before markers
    let (mut b_start, mut b_pos) = (0,0);
    // Proceed extracting delta's
    while b_pos < mapping.len() && a_pos < after.len() {
	match mapping[b_pos] {
	    None => {
		// Uneven case. Increase after buffer
		b_pos += 1;
	    }
	    Some(v) if v < a_pos => {
		// Uneven case. Increase before buffer
		b_pos += 1;		
	    }
	    Some(v) if v > a_pos => {
		// Uneven case. Increase after buffer
		a_pos = v;
	    }
	    Some(_) => {
		// Matching case. Flush buffers and advance
		if b_start < b_pos || a_start < a_pos {
		    let n = b_pos - b_start;
		    println!("ADDING: {a_start} ==> {n}");		    
		    // Extract the difference
		    unsafe { delta.push_raw(a_start .. a_start + n, &after[a_start .. a_pos]); }
		}
		a_pos += 1;
		b_pos += 1;		
		a_start = a_pos;
		b_start = b_pos;
	    }
	}
    }
    // Flush remaining buffers
    if b_start < mapping.len() || a_start < after.len() {
        // Terminating case. Flush buffers and end.
	let n = mapping.len() - b_start;
	println!("ADDING2: {n}");	
	unsafe { delta.push_raw(a_start .. a_start + n, &after[a_start .. ]); }	
    }
    //
    delta
}

// ===================================================================
// Diff Tests
// ===================================================================

#[cfg(test)]
mod diff_tests {
    use std::fmt::Debug;
    use crate::diff::{Diff};
    
    #[test]
    fn test_01() {
	// Empty delta
	check(&[1,2,3],&[1,2,3],0);		
    }
    
    #[test]
    fn test_02() {
	// Addition (1 rewrite)
	check(&[1,2,3],&[4,1,2,3],1);		
    }

    #[test]
    fn test_03() {
	// Addition (1 rewrite)	
	check(&[1,2,3],&[1,4,2,3],1);		
    }

    #[test]
    fn test_04() {
	// Addition (1 rewrite)	
	check(&[1,2,3],&[1,2,4,3],1);		
    }

    #[test]
    fn test_05() {
	// Addition (1 rewrite)		
	check(&[1,2,3],&[1,2,3,4],1);		
    }

    #[test]
    fn test_06() {
	// Addition (1 rewrite)
	check(&[1,2,3],&[4,5,1,2,3],1);		
    }

    #[test]
    fn test_07() {
	// Addition (1 rewrite)	
	check(&[1,2,3],&[1,4,5,2,3],1);		
    }

    #[test]
    fn test_08() {
	// Addition (1 rewrite)	
	check(&[1,2,3],&[1,2,4,5,3],1);		
    }

    #[test]
    fn test_09() {
	// Addition (1 rewrite)		
	check(&[1,2,3],&[1,2,3,4,5],1);		
    }

    #[test]
    fn test_10() {//
	// Replace (1 rewrite)		
	check(&[1,2,3],&[4,2,3],1);		
    }

    #[test]
    fn test_11() { //
	// Replace (1 rewrite)
	check(&[1,2,3],&[1,4,3],1);		
    }

    #[test]
    fn test_12() {
	// Replace (1 rewrite)	
	check(&[1,2,3],&[1,2,4],1);		
    }

    #[test]
    fn test_13() {
	// Replace (1 rewrite)	
	check(&[1,2,3],&[4,5,2,3],1);		
    }

    #[test]
    fn test_14() {
	// Replace (1 rewrite)		
	check(&[1,2,3],&[1,4,5,3],1);		
    }

    #[test]
    fn test_15() {
	// Replace (1 rewrite)		
	check(&[1,2,3],&[1,2,4,5],1);		
    }
    
    #[test]
    fn test_16() {
	// Removal (1 rewrite)	
	check(&[1,2,3],&[2,3],1);
    }

    #[test]
    fn test_17() {
	// Removal (1 rewrite)		
	check(&[1,2,3],&[1,3],1);
    }

    #[test]
    fn test_18() {
	// Removal (1 rewrite)		
	check(&[1,2,3],&[1,2],1);
    }

    #[test]
    fn test_19() {
	// Removal (1 rewrite)		
	check(&[1,2,3],&[3],1);
    }

    #[test]
    fn test_20() {
	// Removal (1 rewrite)		
	check(&[1,2,3],&[1],1);
    }

    #[test]
    fn test_21() {
	// Rewrite everything
	check(&[1,2,3],&[4,5,6],1);		
    }

    #[test]
    fn test_22() {
	// Rewrite everything
	check(&[1,2,3],&[4,5,6,7],1);		
    }

    #[test]
    fn test_23() {
	// Rewrite everything
	check(&[1,2,3],&[4,5],1);		
    }
    
    // Double rewrites

    #[test]
    fn test_40() {
	// Double rewrite
	check(&[1,2,3],&[4,1,2,5,6],2);		
    }

    #[test]
    fn test_41() {
	// Double rewrite
	check(&[1,2,3],&[1,4,2,5,6,3],2);		
    }

    // Triple rewrites
    
    
    // Construct diff between `from` and `to`, which is expected to
    // produce a delta with a given number of rewrites.  Check that
    // applying this delta to `from` produces `to`.
    fn check<T:Clone+Debug+PartialEq>(from: &[T], to: &[T], num: usize) {
	let mut vec = from.to_vec();
	// Generatre diff between `from` and `to`.
	let delta = from.diff(&to);
	//
	println!("GOT: {delta:?}");
	// Check number of rewrites matches expected
	assert_eq!(delta.len(),num);
	// Apply delta to original sequence
	delta.transform(&mut vec);
	// Check matches target sequence
	assert_eq!(&vec,to);
    }
}

// ===================================================================
// LCS Tests
// ===================================================================

#[cfg(test)]
mod lcs_tests {
    use crate::diff::slice::*;

    #[test]
    fn lcs_test_01() {
        let v = longest_common_subsequence::<usize>(&[],&[]);
        assert!(v.is_empty());
    }

    #[test]
    fn lcs_test_02() {
        let v = longest_common_subsequence(&[0],&[]);
        assert_eq!(v,vec![None]);
    }

    #[test]
    fn lcs_test_03() {
        let v = longest_common_subsequence(&[],&[0]);
        assert!(v.is_empty());
    }

    #[test]
    fn lcs_test_04() {
        let v = longest_common_subsequence(&[0],&[0]);
        assert_eq!(v,vec![Some(0)]);
    }

    #[test]
    fn lcs_test_05() {
        let v = longest_common_subsequence(&[0],&[1]);
        assert_eq!(v,vec![None]);
    }

    #[test]
    fn lcs_test_06() {
        let v = longest_common_subsequence(&[0,1],&[0]);
        assert_eq!(v,vec![Some(0),None]);
    }

    #[test]
    fn lcs_test_07() {
        let v = longest_common_subsequence(&[1,0],&[0]);
        assert_eq!(v,vec![None,Some(0)]);
    }

    #[test]
    fn lcs_test_08() {
        let v = longest_common_subsequence(&[0,0],&[0,0]);
        assert_eq!(v,vec![Some(0),Some(1)]);
    }

    #[test]
    fn lcs_test_09() {
        let v = longest_common_subsequence(&[0,1],&[0,0]);
        assert_eq!(v,vec![Some(0),None]);
    }

    #[test]
    fn lcs_test_10() {
        let v = longest_common_subsequence(&[1,0],&[0,0]);
        assert_eq!(v,vec![None,Some(0)]);
    }

    #[test]
    fn lcs_test_11() {
        let v = longest_common_subsequence(&[1,1],&[0,0]);
        assert_eq!(v,vec![None,None]);
    }

    #[test]
    fn lcs_test_20() {
        let v = longest_common_subsequence(&['a','b','b','c','b','c','d'],&['b','b','e','c','d','e']);
        assert_eq!(v,vec![None,Some(0),Some(1),Some(3),None,None,Some(4)]);
    }
}
