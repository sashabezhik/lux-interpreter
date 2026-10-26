use crate::ast::{Token, TokenInfo};

pub struct Lexer {
    input: Vec<char>,
    position: usize,      // current position in input
    read_position: usize, // current reading position (after current char)
    ch: Option<char>,     // current char under examination
    line: usize,          // current line number
    column: usize,        // current column number
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: None,
            line: 1,
            column: 0,
        };
        lexer.read_char();

        lexer
    }

    /// Main tokenization method
    pub fn parse_input_into_tokens(&mut self) -> Result<Vec<TokenInfo>, String> {
        let mut tokens = Vec::new();

        loop {
            self.skip_whitespace_and_comments();

            let token_line = self.line;
            let token_column = self.column;

            let token = match self.ch {
                None => Token::Eof,

                // Single-character tokens
                Some('(') => {
                    self.read_char();
                    Token::LeftParen
                }

                Some(')') => {
                    self.read_char();
                    Token::RightParen
                }

                Some('{') => {
                    self.read_char();
                    Token::LeftBrace
                }

                Some('}') => {
                    self.read_char();
                    Token::RightBrace
                }

                Some('[') => {
                    self.read_char();
                    Token::LeftBracket
                }

                Some(']') => {
                    self.read_char();
                    Token::RightBracket
                }

                Some(',') => {
                    self.read_char();
                    Token::Comma
                }

                Some(';') => {
                    self.read_char();
                    Token::Semicolon
                }

                Some('+') => {
                    self.read_char();
                    Token::Plus
                }

                Some('-') => {
                    self.read_char();
                    Token::Minus
                }

                Some('*') => {
                    self.read_char();
                    Token::Star
                }

                Some('/') => {
                    self.read_char();
                    Token::Slash
                }

                Some('%') => {
                    self.read_char();
                    Token::Percent
                }

                // One or two character tokens
                Some('=') => {
                    if self.peek_char() == Some('=') {
                        self.read_char();
                        self.read_char();

                        Token::EqualEqual
                    } else {
                        self.read_char();
                        Token::Equal
                    }
                }

                Some('!') => {
                    if self.peek_char() == Some('=') {
                        self.read_char();
                        self.read_char();

                        Token::BangEqual
                    } else {
                        self.read_char();
                        Token::Bang
                    }
                }

                Some('<') => {
                    if self.peek_char() == Some('=') {
                        self.read_char();
                        self.read_char();

                        Token::LessEqual
                    } else {
                        self.read_char();
                        Token::Less
                    }
                }

                Some('>') => {
                    if self.peek_char() == Some('=') {
                        self.read_char();
                        self.read_char();

                        Token::GreaterEqual
                    } else {
                        self.read_char();
                        Token::Greater
                    }
                }

                // String literals
                Some('"') => self.read_string()?,

                // Number literals
                Some(c) if c.is_ascii_digit() => self.read_number()?,

                // Identifiers and keywords
                Some(c) if c.is_alphabetic() || c == '_' => self.read_identifier(),

                Some(c) => {
                    return Err(format!(
                        "Unexpected character '{}' at line {}, column {}",
                        c, self.line, self.column,
                    ));
                }
            };

            if token == Token::Eof {
                tokens.push(TokenInfo {
                    token,
                    line: token_line,
                    column: token_column,
                });
                break;
            }

            tokens.push(TokenInfo {
                token,
                line: token_line,
                column: token_column,
            });
        }

        Ok(tokens)
    }

    /// Read the next character and advance position
    fn read_char(&mut self) {
        if self.is_last_char() {
            self.ch = None;
        } else {
            self.ch = Some(self.input[self.read_position]);
        }

        self.position = self.read_position;
        self.read_position += 1;
        self.column += 1;
    }

    /// Peek at the next character without advancing
    fn peek_char(&self) -> Option<char> {
        if self.is_last_char() {
            None
        } else {
            Some(self.input[self.read_position])
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.ch {
                Some(' ') | Some('\t') | Some('\r') => {
                    self.read_char();
                }
                Some('\n') => {
                    self.next_line();
                    self.read_char();
                }
                Some('/') if self.peek_char() == Some('/') => {
                    // Line comment
                    while self.ch.is_some() && self.ch != Some('\n') {
                        self.read_char();
                    }
                }
                Some('/') if self.peek_char() == Some('*') => {
                    // Block comment
                    self.read_char();
                    self.read_char();

                    while self.ch.is_some() {
                        if self.ch == Some('*') && self.peek_char() == Some('/') {
                            self.read_char();
                            self.read_char();

                            break;
                        }
                        if self.ch == Some('\n') {
                            self.next_line();
                        }

                        self.read_char();
                    }
                }
                _ => break,
            }
        }
    }

    /// Read a string literal
    fn read_string(&mut self) -> Result<Token, String> {
        self.read_char();

        let start_line = self.line;
        let mut value = String::new();

        while self.ch.is_some() && self.ch != Some('"') {
            if self.ch == Some('\\') {
                self.read_char();

                match self.ch {
                    Some('n') => value.push('\n'),
                    Some('t') => value.push('\t'),
                    Some('r') => value.push('\r'),
                    Some('\\') => value.push('\\'),
                    Some('"') => value.push('"'),
                    Some(c) => {
                        return Err(format!(
                            "Invalid escape sequence '\\{}' at line {}",
                            c, self.line,
                        ));
                    }
                    None => {
                        return Err(format!("Unterminated string at line {}", start_line));
                    }
                }

                self.read_char();
            } else {
                if self.ch == Some('\n') {
                    self.next_line();
                }

                value.push(self.ch.unwrap());
                self.read_char();
            }
        }

        if self.ch.is_none() {
            return Err(format!("Unterminated string at line {}", start_line));
        }

        self.read_char();
        Ok(Token::String(value))
    }

    /// Read a number literal (integer or float)
    fn read_number(&mut self) -> Result<Token, String> {
        let mut number = String::new();

        while let Some(c) = self.ch {
            if c.is_ascii_digit() {
                number.push(c);
                self.read_char();
            } else if c == '.' && self.is_next_char_ascii_digit() {
                number.push(c);
                self.read_char();
            } else {
                break;
            }
        }

        number
            .parse()
            .map(Token::Number)
            .map_err(|_| format!("Invalid number: {}", number))
    }

    /// Read an identifier or a keyword
    fn read_identifier(&mut self) -> Token {
        let mut identifier = String::new();

        while let Some(c) = self.ch {
            if c.is_alphabetic() || c == '_' {
                identifier.push(c);
                self.read_char();
            } else {
                break;
            }
        }

        match identifier.as_str() {
            "let" => Token::Let,
            "fn" => Token::Fn,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "for" => Token::For,
            "in" => Token::In,
            "return" => Token::Return,
            "true" => Token::True,
            "false" => Token::False,
            "nil" => Token::Nil,
            "print" => Token::Print,
            "and" => Token::And,
            "or" => Token::Or,
            "not" => Token::Not,
            _ => Token::Identifier(identifier),
        }
    }

    fn next_line(&mut self) {
        self.line += 1;
        self.column = 0;
    }

    fn is_last_char(&self) -> bool {
        self.read_position >= self.input.len()
    }

    fn is_next_char_ascii_digit(&self) -> bool {
        self.peek_char()
            .map(|char| char.is_ascii_digit())
            .unwrap_or(false)
    }
}
