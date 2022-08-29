use delta_inc::lex::{Span,Tokeniser,Tokenisation};

// ==================================================================
// Token
// ==================================================================

#[derive(PartialEq,Debug)]
pub enum TokenKind {
    LeftBrace,
    RightBrace,
    Number,
    Identifier
}

#[derive(PartialEq,Debug)]
struct Token { kind: TokenKind, start: usize, end: usize }

impl Token {
    pub fn new(kind: TokenKind, start: usize, end: usize) -> Self {
        Token{kind,start,end}
    }
}

impl Span for Token {
    fn start(&self) -> usize { self.start }
    fn end(&self) -> usize { self.end }
}

// ==================================================================
// Char Tokeniser
// ==================================================================

struct CharTokeniser();

impl CharTokeniser {
    /// Scan a sequence of one or more digits starting at a given
    /// position in the underlying sequence.
    fn scan_number(&self, items: &[char], index: usize) -> Result<Token,()> {
        let mut i = index + 1;
        // Scan all digits.
        while i < items.len() && items[i].is_digit(10) { i = i + 1; }
        // Done
        Ok(Token::new(TokenKind::Number,index,i-1))
    }

    /// Scan an identifier which has the form
    /// `(a-zA-Z_)[a-zA-Z0-9_]*`.  That is, its a sequence beginning
    /// with an alphabetic character or an underscore, followed a
    /// sequence of zero or more characters which are either
    /// alphabetic, numeric or an underscore.
    fn scan_identifier(&self, items: &[char], index: usize) -> Result<Token,()> {
        let mut i = index + 1;
        // Scan all digits.
        while i < items.len() && Self::is_identifier_middle(items[i]) {
            i = i + 1;
        }
        // Done
        Ok(Token::new(TokenKind::Identifier,index,i-1))
    }

    /// Determine whether a given character can occur in the middle of an
    /// identifier
    fn is_identifier_middle(c: char) -> bool {
        c.is_digit(10) || c.is_ascii_alphabetic() || c == '_'
    }
}

impl Tokeniser for CharTokeniser {
    /// Define input type for this tokenizer
    type Input = char;
    /// Define output type for this tokenizer
    type Output = Token;
    /// Define error type for this tokenizer
    type Error = ();

    fn scan(&self, items: &[char], i: usize) -> Result<Token,Self::Error> {
        if i >= items.len() {
            Err(())
        } else {
            match items[i] {
                '(' => Ok(Token::new(TokenKind::LeftBrace,i,i)),
                ')' => Ok(Token::new(TokenKind::RightBrace,i,i)),
                '0'..='9' => self.scan_number(items,i),
                'a'..='z'|'A'..='Z'|'_' => self.scan_identifier(items,i),
                _ => Err(())
            }
        }
    }
}

// ============================================================================
// Test (Tokeniser)
// ============================================================================

#[test]
fn test_tokenizer_01() {
    let t = CharTokeniser();
    assert!(t.scan(&['*'],0).is_err());
}

#[test]
fn test_tokenizer_02() {
    let token = CharTokeniser().scan(&['('],0).unwrap();
    assert!(token == Token::new(TokenKind::LeftBrace,0,0));
}

#[test]
fn test_tokenizer_03() {
    let token = CharTokeniser().scan(&['a',')'],1).unwrap();
    assert!(token == Token::new(TokenKind::RightBrace,1,1));
}

#[test]
fn test_tokenizer_04() {
    let token = CharTokeniser().scan(&['0','1'],0).unwrap();
    assert!(token == Token::new(TokenKind::Number,0,1));
}

#[test]
fn test_tokenizer_05() {
    let token = CharTokeniser().scan(&['0','1','a'],0).unwrap();
    assert!(token == Token::new(TokenKind::Number,0,1));
}

#[test]
fn test_tokenizer_06() {
    let token = CharTokeniser().scan(&['a'],0).unwrap();
    assert!(token == Token::new(TokenKind::Identifier,0,0));
}

#[test]
fn test_tokenizer_07() {
    let token = CharTokeniser().scan(&['a','b','c'],0).unwrap();
    assert!(token == Token::new(TokenKind::Identifier,0,2));
}

#[test]
fn test_tokenizer_08() {
    let token = CharTokeniser().scan(&['_','b','c'],0).unwrap();
    assert!(token == Token::new(TokenKind::Identifier,0,2));
}

// ============================================================================
// Test (Lexer)
// ============================================================================

#[test]
fn test_lexer_01() {
    let tizer = Tokenisation::new(vec!['a','b','1'],CharTokeniser());
    assert!(tizer.is_ok());
}

#[test]
fn test_lexer_02() {
    let tizer = Tokenisation::new(vec!['a','b','*'],CharTokeniser());
    assert!(tizer.is_err());
}

#[test]
fn test_lexer_03() {
    // Create iterator
    let tizer = Tokenisation::new(vec!['1','a','b'],CharTokeniser()).unwrap();
    let tokens : Vec<Token> = tizer.iter().collect();
    assert!(tokens.len() == 2);
    assert!(tokens[0] == Token::new(TokenKind::Number,0,0));
    assert!(tokens[1] == Token::new(TokenKind::Identifier,1,2));
}

#[test]
fn test_lexer_04() {
    // Create iterator
    let tizer = Tokenisation::new(vec!['1','a','b'],CharTokeniser()).unwrap();
    for t in &tizer {
        println!("GOT {:?}",t);
    }
    let tokens : Vec<Token> = tizer.iter().collect();
    assert!(tokens.len() == 2);
}
