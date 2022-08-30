use std::iter::Iterator;
use crate::vec;
use crate::{PartiallyTransformable,Transformable};

/// ==================================================================
/// Span
/// ==================================================================

/// An abstract notion of a token which groups one or more items in
/// an underlying sequence together.
pub trait Span {
    /// Get the starting index of this token in the underlying
    /// sequence.
    fn start(&self) -> usize;
    /// Get the last index of this token in the underlying sequence.
    /// Observe that tokens cannot be "zero sized", hence `start ==
    /// end` represents a token of size one.
    fn end(&self) -> usize;
}

/// ==================================================================
/// Tokeniser
/// ==================================================================

/// Represents something which maps one (or more) items from an
/// _underlying sequence_ to a given _token_ (or an error).  In essence,
/// we are "folding" one or more items into a single token (which, for
/// example, is a useful characterisation for lexers).
///
/// The challenge is that we don't know ahead of time how many items
/// are required for each token.  As such, the folding "consumes" some
/// number of items from the stream.
pub trait Tokeniser {
    /// The type of items in the underlying sequence.
    type Input;
    /// The type of tokens produced by this tokeniser.
    type Output:Span;
    /// Identifies the kind of error which can arise during
    /// tokenisation.
    type Error;
    /// Get the `Token` starting at a given `index` in an underlying
    /// sequence.
    fn scan(&self, seq: &[Self::Input], index: usize) -> Result<Self::Output,Self::Error>;
}


/// ==================================================================
/// Token Iterator
/// ==================================================================
pub struct TokenIterator<'a, T : Tokeniser> {
    /// Slice of items to tokenise
    items: &'a [T::Input],
    /// Current position within slice
    index: usize,
    /// Function responsible for tokenisation
    tokeniser: &'a T
}

impl<'a,T:Tokeniser> TokenIterator<'a,T> {
    pub fn new(items: &'a [T::Input], tokeniser: &'a T) -> TokenIterator<'a,T> {
        TokenIterator{items,index:0,tokeniser}
    }
}

impl<'a,T:Tokeniser> Iterator for TokenIterator<'a,T> {
    type Item = T::Output;

    /// Scan next token from the input sequence.
    fn next(&mut self) -> Option<Self::Item> {
        // Run the tokeniser.
	match self.tokeniser.scan(self.items,self.index) {
            Ok(t) => {
                // Move iterator
                self.index = t.end() + 1;
                Some(t)
            }
            Err(_) => None
        }
    }
}

/// ==================================================================
/// Tokenisation
/// ==================================================================

/// Represents the results of a _lexical analysis_ of a sequence of
/// items (henceforth, the _underlying sequence_) to a sequence of
/// _tokens_.  This is characterised as a _data structure_
/// (i.e. rather than a _function_ which would be more common) to
/// faciliate to facilitate incremental updates.  Since tokens can
/// span multiple items in the underlying sequence, managing
/// incremental updates is non-trivial.  Furthermore, there is a
/// built-in assumption that tokenisation can fail (i.e. there are
/// sequences of items which do not map to any tokens).
///
/// For example, lets consider the common case of turning character
/// sequences into tokens (i.e. _lexing_ as performed by a compiler).
/// Suppose tokens are either _identifiers_ (i.e. sequences of
/// alphabetic characters), _numbers_ (i.e. sequences of numeric
/// characters) or _operators_ (e.g. braces).  Furthermore, not all
/// characters map to tokens, and thus lexing can fail with an error.
/// To manage incremental updates efficiently the tokeniser internally
/// maintains meta-data which identifies token boundaries:
///
/// ```text
///         0 1 2 3 4 5 6 7 8 9
///         +-+-+-+-+-+-+-+-+-+-+
/// bytes:  |(|1|2|3|)|h|e|l|l|o|
///         +-+-+-+-+-+-+-+-+-+-+
///          | |     | |
///          | |     | |
///         +-+-+-+-+-+-+-+-+-+-+
/// starts: |*|*| | |*|*| | | | |
///         +-+-+-+-+-+-+-+-+-+-+
/// ```
/// Here we see the starting boundaries of each token identified by
/// `*` (which corresponds with `true`).
pub struct Tokenisation<T : Tokeniser> {
    /// The underlying sequence being (incrementally) tokenised by
    /// this lexer.
    items: Vec<T::Input>,
    /// The underlying tokeniser which is responsible for turing
    /// inputs into outputs.
    tokeniser: T,
    /// Meta-data which identifies token boundaries.  More
    /// specifically, <code>true</code> indicates the corresponding
    /// item (in the underlying sequence) is the start of a token.
    starts: Vec<bool>
}

impl<T:Tokeniser> Tokenisation<T> {
    /// Construct an iterator from this tokenisation.
    pub fn iter(&self) -> TokenIterator<T> {
        TokenIterator::new(&self.items,&self.tokeniser)
    }
}

impl<T: Tokeniser> Tokenisation<T> {
    /// Construct an incremental lexer from a tokeniser.  This
    /// immediately tokenises the stream and, hence, can fail with an
    /// error (e.g. if an unknown sequence is encountered).
    pub fn new(items: Vec<T::Input>, tokeniser: T) -> Result<Tokenisation<T>, T::Error> {
        // Construct meta-data from scratch.
        let mut starts = Self::generate_starts(&items,&tokeniser)?;
        // Done
        Ok(Tokenisation{items,tokeniser,starts})
    }

    /// Validate that the meta-data is up-to-date with the underlying
    /// sequence.  This is, in essence, a safety check which can be
    /// run after a transformation has been applied to check
    /// everything still makes sense.
    pub fn validate(&self) -> Result<(),T::Error> {
        // Regenerate meta-data from scratch.
        let nstarts = Self::generate_starts(&self.items,&self.tokeniser)?;
        // Check it is the same.
        assert!(nstarts == self.starts);
        //
        Ok(())
    }

    /// Generate meta-data (i.e. `starts`) for this tokeniser from
    /// scratch.
    fn generate_starts(items: &Vec<T::Input>, tokeniser: &T) -> Result<Vec<bool>, T::Error> {
        let mut starts = std::vec![false; items.len()];
        // Perform the tokenisation
        let mut i = 0;
        while i < items.len() {
            // Start of token
            starts[i] = true;
            // scan it
            let t = tokeniser.scan(&items,i)?;
            // Move on
            i = t.end()+1;
        }
        // Done
        Ok(starts)
    }
}

/// Straightforward conversion from a `Tokenisation` to an `Iterator`.
impl<'a,T:Tokeniser> IntoIterator for &'a Tokenisation<T> {
    type Item = T::Output;
    type IntoIter = TokenIterator<'a,T>;

    fn into_iter(self) -> Self::IntoIter { self.iter() }
}

/// ==================================================================
/// Transformable
/// ==================================================================

/// Allow a tokenisation to be incrementally updated through a
/// _transformation_ on the underlying sequence.
impl<T:Tokeniser> PartiallyTransformable for Tokenisation<T>
where T::Input: Clone {
    /// A tokenisation delta corresponds to a delta on the underlying
    /// input sequence.  They key is that applying this delta to the
    /// tokenisation requires that it _incrementally updates_ the
    /// associated meta-data.
    type Delta = vec::Delta<T::Input>;
    /// Define the type of errors which can arise from an invalid
    /// transformation.
    type Error = T::Error;
    /// Transform a tokenisation in place.
    fn transform(&mut self,d: &Self::Delta) -> Result<(),Self::Error> {
        // Transform the underlying items.
        self.items.transform(d);
        // Construct starts delta
        // FIXME: this is not efficient.
        let rws : Vec<vec::Rewrite<bool>> = d.iter().map(|r| r.map(|i| false)).collect();
        let sd : vec::Delta<bool> = vec::Delta::from(rws);
        // Apply starts delta
        self.starts.transform(&sd);
        // Sanity check.
        assert!(self.starts.len() == self.items.len());
        // Transform starts
        self.starts = Self::generate_starts(&self.items,&self.tokeniser)?;
        // All good!
        Ok(())
    }
}
