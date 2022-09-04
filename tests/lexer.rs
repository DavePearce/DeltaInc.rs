use delta_inc::{PartiallyTransformable};
use delta_inc::lex;
use delta_inc::lex::{SnapResult,Scanner,Span,TableTokenizer};

// =================================================================
// Token
// =================================================================
#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Token {
    EOF,
    Identifier,
    LeftBrace,
    Number,
    RightBrace
}

// ======================================================
// Rules
// ======================================================

/// Handy type alias for the result type used for all of the lexical
/// rules.
type Result = std::result::Result<Span<Token>,()>;

/// Scan a numeric literal.
fn scan_number(input: &[char]) -> Result {
    scan_whilst(input, Token::Number, |c| c.is_digit(10))
}

/// Scan an identifier which starts with an alpabetic character, or an
/// underscore and subsequently contains zero or more alpha-number
/// characters or underscores.
fn scan_identifier(input: &[char]) -> Result {
    if input.len() > 0 && is_identifier_start(input[0]) {
        scan_whilst(input, Token::Identifier, is_identifier_middle)
    } else {
        Err(())
    }
}

/// Scan all single-character operators.
fn scan_brace_operators(input: &[char]) -> Result {
    if input.len() == 0 {
        Err(())
    } else {
        let t = match input[0] {
            '(' => Token::LeftBrace,
            ')' => Token::RightBrace,
            _ => { return Err(()); }
        };
        //
        Ok(Span::new(t,0..1))
    }
}

/// Determine whether a given character is the start of an identifier.
fn is_identifier_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

/// Determine whether a given character can occur in the middle of an
/// identifier
fn is_identifier_middle(c: char) -> bool {
    c.is_digit(10) || is_identifier_start(c)
}

/// If there is nothing left to scan, then we've reached the
/// End-Of-File.
fn scan_eof(input: &[char]) -> Result {
    if input.len() == 0 {
        Ok(Span::new(Token::EOF,0..0))
    } else {
        Err(())
    }
}

/// Helper which scans an item matching a given predicate.  If no
/// characters match, then it fails.
fn scan_whilst<P>(input: &[char], t: Token, pred: P) -> Result
where P: Fn(char) -> bool {
    let mut i = 0;
    // Continue whilst predicate matches
    while i < input.len() && pred(input[i]) { i = i + 1; }
    // Check what happened
    if i == 0 {
        // Nothing matched
        Err(())
    } else {
        // Something matched
        Ok(Span::new(t, 0..i))
    }
}

/// The set of rules used for lexing.
static RULES : &'static [Scanner<char,Token>] = &[
    scan_brace_operators,
    scan_identifier,
    scan_number,
    scan_eof
];

// ======================================================
// Lexer
// ======================================================

pub struct Lexer {
    /// Internal lexer used for the heavy lifting.
    lexer: lex::Lexer<TableTokenizer<char,Token>>
}

impl Lexer {
    /// Construct a `Lexer` from a given string slice.
    pub fn new(input: &str) -> Lexer {
        let tokenizer = TableTokenizer::new(RULES.to_vec());
        let chars = input.chars().collect();
        Lexer{lexer:lex::Lexer::new(chars, tokenizer)}
    }

    pub fn get(&self, t: Span<Token>) -> &[char] {
        self.lexer.get(t)
    }

    /// Pass through request to underlying lexer
    pub fn is_eof(&self) -> bool { self.lexer.is_eof() }
    /// Pass through request to underlying lexer
    pub fn peek(&self) -> Span<Token> { self.lexer.peek() }
    /// Pass through request to underlying lexer
    pub fn snap(&mut self, kind : Token) -> SnapResult<Token> {
        self.lexer.snap(kind)
    }
    /// Pass through request to underlying lexer
    pub fn snap_any(&mut self, kinds : &[Token]) -> SnapResult<Token> {
        self.lexer.snap_any(kinds)
    }
}

// ============================================================================
// Test (Tokeniser)
// ============================================================================


/// Handy definition
macro_rules! assert_ok {
    ($result:expr) => { assert!($result.is_ok()); };
}

#[test]
fn test_tokenizer_01() {
    let mut l = Lexer::new("()");
    assert_ok!(l.snap(Token::LeftBrace));
    assert_ok!(l.snap(Token::RightBrace));
    assert_ok!(l.snap(Token::EOF));
}

#[test]
fn test_tokenizer_02() {
    let mut l = Lexer::new("01");
    assert_ok!(l.snap(Token::Number));
    assert_ok!(l.snap(Token::EOF));
}

#[test]
fn test_tokenizer_03() {
    let mut l = Lexer::new("abc");
    assert_ok!(l.snap(Token::Identifier));
    assert_ok!(l.snap(Token::EOF));
}

#[test]
fn test_tokenizer_04() {
    let mut l = Lexer::new("0123(abc");
    assert_ok!(l.snap(Token::Number));
    assert_ok!(l.snap(Token::LeftBrace));
    assert_ok!(l.snap(Token::Identifier));
    assert_ok!(l.snap(Token::EOF));
}
