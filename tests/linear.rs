use delta_inc::linear::{Span};

/// A simple lineriser which divides sequences based on some specific
/// item.
struct Splitter<'a,T> {
    /// Represents the underlying sequence.
    seq: &'a [T],
    /// Item to split on!
    item: T,
    /// Current index into underlying sequence.
    index: usize
}

impl<'a,T> Splitter<'a,T>
where T:PartialEq {
    pub fn new(seq: &'a [T], item: T) -> Self {
        Self{seq,item,index:0}
    }

    pub fn eof(&self) -> bool { self.index >= self.seq.len() }

    pub fn advance(&mut self) -> usize {
        let mut i = self.index;
        // Skip separator (if applicable)
        if !self.eof() && self.seq[i] == self.item {
            i = i+1;
        }
        // Save start
        let start = i;
        // Continue until next boundary
        while i < self.seq.len() && self.seq[i] != self.item {
            i = i + 1;
        }
        // Update start position
        self.index = i;
        // Done
        start
    }
}

impl<'a,T:PartialEq> Iterator for Splitter<'a,T> {
    type Item = Span<&'a [T]>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.eof() {
            None
        } else {
            // Continue until next boundary
            let start = self.advance();
            // Construct span
            let region = start .. self.index;
            // Construct item
            let item = &self.seq[start .. self.index];
            // Done
            Some(Span{region,item})
        }
    }
}

#[test]
fn test_lineariser_01() {
    let s = Splitter::new(&[1,2,3],0);
    let v : Vec<Span<&[usize]>> = s.collect();
    assert!(v.len() == 1);
    assert_eq!(v[0].region,0..3);
    assert_eq!(v[0].item,&[1,2,3]);
}

#[test]
fn test_lineariser_02() {
    let s = Splitter::new(&[1,2,0,3],0);
    let v : Vec<Span<&[usize]>> = s.collect();
    assert!(v.len() == 2);
    assert_eq!(v[0].region,0..2);
    assert_eq!(v[0].item,&[1,2]);
    assert_eq!(v[1].region,3..4);
    assert_eq!(v[1].item,&[3]);
}
