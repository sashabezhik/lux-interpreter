#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Single-character tokens
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Comma,        // ,
    Semicolon,    // ;
    Plus,         // +
    Minus,        // -
    Star,         // *
    Slash,        // /
    Percent,      // %

    // One or two character tokens
    Equal,        // =
    EqualEqual,   // ==
    Bang,         // !
    BangEqual,    // !=
    Less,         // <
    LessEqual,    // <=
    Greater,      // >
    GreaterEqual, // >=

    // Literals
    Identifier(String),
    String(String),
    Number(f64),

    // Keywords
    Let,
    Fn,
    If,
    Else,
    While,
    For,
    In,
    Return,
    True,
    False,
    Nil,
    Print,
    And,
    Or,
    Not,

    // Special
    Eof,
}

#[derive(Debug)]
pub struct TokenInfo {
    pub token: Token,
    pub line: usize,
    pub column: usize,
}
