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

// Expression nodes
#[derive(Debug)]
pub enum Expr {
    // Literals
    Number(f64),
    String(String),
    Bool(bool),
    Nil,

    // Variable reference
    Identifier(String),

    // Unary operation: -x, not x
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },

    // Binary operation: x + y, x == y
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },

    // Grouping: (expr)
    Grouping(Box<Expr>),

    // Assignment: x = value
    Assign {
        name: String,
        value: Box<Expr>,
    },

    // Function call: func(arg1, arg2)
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },

    // List indexing: list[index]
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },

    // List literal: [1, 2, 3]
    List(Vec<Expr>),

    // Anonymous function: fn(x) { return x * 2; }
    Lambda {
        params: Vec<String>,
        body: Vec<Stmt>,
    },
}

// Statement nodes
#[derive(Debug)]
pub enum Stmt {
    // Expression statement: expr;
    Expression(Expr),

    // Variable declaration: let x = expr;
    Let {
        name: String,
        value: Expr,
    },

    // Print statement: print expr;
    Print(Expr),

    // Block: { stmt1; stmt2; }
    Block(Vec<Stmt>),

    // If statement: if expr { block } else { block }
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },

    // While loop: while expr { block }
    While {
        condition: Expr,
        body: Box<Stmt>,
    },

    // For loop: for item in iterable { block }
    For {
        variable: String,
        iterable: Expr,
        body: Box<Stmt>,
    },

    // Function declaration: fn name(params) { body }
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },

    // Return statement: return expr;
    Return(Option<Expr>),
}

// Operators
#[derive(Debug)]
pub enum UnaryOp {
    Minus, // -
    Not,   // not
}

#[derive(Debug)]
pub enum BinaryOp {
    // Arithmetic
    Add,      // +
    Subtract, // -
    Multiply, // *
    Divide,   // /
    Modulo,   // %

    // Comparison
    Equal,        // =
    NotEqual,     // !=
    Less,         // <
    LessEqual,    // <=
    Greater,      // >
    GreaterEqual, // >=

    // Logical
    And, // and
    Or,  // or
}
