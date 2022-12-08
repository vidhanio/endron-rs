#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub end: usize,
    pub column: usize,
    pub line: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Eof,
    Ident,
    LineComment,
    DocComment,
    LiteralDecimal,
    LiteralNumber,
    LiteralString,

    LBrace,
    RBrace,
    LParen,
    RParen,

    Comma,
    Semicolon,

    Caret,
    Colon,
    Period,
    Underscore,

    At,
    Bang,
    Dollar,
    Pipe,
    QuestionMark,
    Tilde,

    NumberSign,
    Plus,
    Minus,
    Asterisk,
    DoubleAsterisk,
    Slash,
    GreaterThan,
    LessThan,
    Equal,
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut tokenizer = Tokenizer::new(source);

    loop {
        let token = tokenizer.next();
        tokenizer.tokens.push(token);
        if token.kind == TokenKind::Eof {
            return tokenizer.tokens;
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
struct Tokenizer<'s> {
    pub source: &'s [u8],
    pub index: usize,
    pub line: usize,
    pub column: usize,
    pub tokens: Vec<Token>,
}

impl<'s> Tokenizer<'s> {
    fn new(source: &'s str) -> Self {
        Self {
            source: source.as_bytes(),
            ..Default::default()
        }
    }

    fn next(&mut self) -> Token {
        let mut state = TokenizerState::Start;
        let mut res = Token {
            kind: TokenKind::Eof,
            start: self.index,
            end: self.index,
            column: self.column,
            line: self.line,
        };

        while self.index < self.source.len() {
            let c = self.source[self.index];

            match state {
                TokenizerState::Start => match c {
                    b' ' => {
                        res.column += 1;
                        res.start += 1;
                        self.column += 1;
                        self.index += 1;
                    }
                    b'\n' => {
                        res.line += 1;
                        res.column = 0;
                        res.start += 1;
                        self.line += 1;
                        self.column = 0;
                        self.index += 1;
                    }
                    b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                        state = TokenizerState::Ident;
                        res.kind = TokenKind::Ident;
                        self.column += 1;
                        self.index += 1;
                    }
                    b'0'..=b'9' => {
                        state = TokenizerState::Number;
                        res.kind = TokenKind::LiteralNumber;
                        self.column += 1;
                        self.index += 1;
                    }
                    b'/' => {
                        state = TokenizerState::Slash;
                        res.kind = TokenKind::Slash;
                        self.column += 1;
                        self.index += 1;
                    }
                    b'"' => {
                        state = TokenizerState::String;
                        res.kind = TokenKind::LiteralString;
                        self.column += 1;
                        self.index += 1;
                    }
                    b'{' => {
                        res.kind = TokenKind::LBrace;
                        self.column += 1;
                        self.index += 1;
                        break;
                    }
                    b'}' => {
                        res.kind = TokenKind::RBrace;
                        self.column += 1;
                        self.index += 1;
                        break;
                    }
                    b'(' => {
                        res.kind = TokenKind::LParen;
                        self.column += 1;
                        self.index += 1;
                        break;
                    }
                    b')' => {
                        res.kind = TokenKind::RParen;
                        self.column += 1;
                        self.index += 1;
                        break;
                    }
                    b',' => {
                        res.kind = TokenKind::Comma;
                        self.column += 1;
                        self.index += 1;
                        break;
                    }
                    b';' => {
                        res.kind = TokenKind::Semicolon;
                        self.column += 1;
                        self.index += 1;
                        break;
                    }
                    b'^' => {
                        res.kind = TokenKind::Caret;
                        self.column += 1;
                        self.index += 1;
                        break;
                    }
                    b':' => {
                        res.kind = TokenKind::Colon;
                        self.column += 1;
                        self.index += 1;
                        break;
                    }
                    b'.' => {
                        res.kind = TokenKind::Period;
                        self.column += 1;
                        self.index += 1;
                        break;
                    }
                    b'@' => {
                        res.kind = TokenKind::At;
                        self.column += 1;
                        self.index += 1;
                        break;
                    }
                    b'!' => {
                        res.kind = TokenKind::Bang;
                        self.column += 1;
                        self.index += 1;
                        break;
                    }
                    b'$' => {
                        res.kind = TokenKind::Dollar;
                        self.column += 1;
                        self.index += 1;
                        break;
                    }
                    b'|' => {
                        res.kind = TokenKind::Pipe;
                        self.column += 1;
                        self.index += 1;
                        break;
                    }
                    b'?' => {
                        res.kind = TokenKind::QuestionMark;
                        self.column += 1;
                        self.index += 1;
                        break;
                    }
                    b'~' => {
                        res.kind = TokenKind::Tilde;
                        self.column += 1;
                        self.index += 1;
                        break;
                    }
                    b'#' => {
                        res.kind = TokenKind::NumberSign;
                        self.column += 1;
                        self.index += 1;
                        break;
                    }
                    b'+' => {
                        res.kind = TokenKind::Plus;
                        self.column += 1;
                        self.index += 1;
                        break;
                    }
                    b'-' => {
                        state = TokenizerState::Minus;
                        res.kind = TokenKind::Minus;
                        self.column += 1;
                        self.index += 1;
                    }
                    b'*' => {
                        state = TokenizerState::Asterisk;
                        res.kind = TokenKind::Asterisk;
                        self.column += 1;
                        self.index += 1;
                    }
                    b'>' => {
                        res.kind = TokenKind::GreaterThan;
                        self.column += 1;
                        self.index += 1;
                        break;
                    }
                    b'<' => {
                        res.kind = TokenKind::LessThan;
                        self.column += 1;
                        self.index += 1;
                        break;
                    }
                    b'=' => {
                        res.kind = TokenKind::Equal;
                        self.column += 1;
                        self.index += 1;
                        break;
                    }
                    _ => panic!(
                        "invalid character: {} at {}:{}",
                        char::from(c),
                        self.line,
                        self.column
                    ),
                },
                TokenizerState::Ident => match c {
                    b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' => {
                        self.column += 1;
                        self.index += 1;
                    }
                    _ => {
                        if &self.source[res.start..self.index] == b"_" {
                            res.kind = TokenKind::Underscore;
                        }
                        break;
                    }
                },
                TokenizerState::Minus => match c {
                    b'0'..=b'9' => {
                        state = TokenizerState::Number;
                        res.kind = TokenKind::LiteralNumber;
                        self.column += 1;
                        self.index += 1;
                    }
                    _ => break,
                },
                TokenizerState::Number => match c {
                    b'0'..=b'9' => {
                        self.column += 1;
                        self.index += 1;
                    }
                    b'.' => {
                        state = TokenizerState::Decimal;
                        res.kind = TokenKind::LiteralDecimal;
                        self.column += 1;
                        self.index += 1;
                    }
                    _ => break,
                },
                TokenizerState::Decimal => match c {
                    b'0'..=b'9' => {
                        self.column += 1;
                        self.index += 1;
                    }
                    _ => break,
                },
                TokenizerState::String => match c {
                    b'"' => {
                        self.column += 1;
                        self.index += 1;
                        break;
                    }
                    b'\\' => {
                        self.column += 2;
                        self.index += 2;
                    }
                    b'\n' => {
                        self.line += 1;
                        self.column = 0;
                        self.index += 1;
                    }
                    _ => {
                        self.column += 1;
                        self.index += 1;
                    }
                },
                TokenizerState::Slash => match c {
                    b'/' => {
                        state = TokenizerState::LineComment;
                        res.kind = TokenKind::LineComment;
                        self.column += 1;
                        self.index += 1;
                    }
                    _ => break,
                },
                TokenizerState::LineComment => match c {
                    b'/' => {
                        state = TokenizerState::DocComment;
                        res.kind = TokenKind::DocComment;
                        self.column += 1;
                        self.index += 1;
                    }
                    b'\n' => break,
                    _ => {
                        self.column += 1;
                        self.index += 1;
                    }
                },
                TokenizerState::DocComment => match c {
                    b'\n' => break,
                    _ => {
                        self.column += 1;
                        self.index += 1;
                    }
                },
                TokenizerState::Asterisk => match c {
                    b'*' => {
                        res.kind = TokenKind::DoubleAsterisk;
                        self.column += 1;
                        self.index += 1;
                        break;
                    }
                    _ => break,
                },
                _ => unreachable!(),
            }
        }

        res.end = self.index;
        res.column += 1;
        res.line += 1;

        res
    }
}

pub enum TokenizerState {
    Start,

    Ident,

    String,
    Zero,
    Minus,
    Number,
    Decimal,

    Slash,
    LineComment,
    DocComment,

    Asterisk,
}
