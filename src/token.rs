use std::fmt::{Display, Formatter};
#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum Token<'a> {
    Eof,
    Identifier(&'a str),
    Integer(&'a str),
    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LessThan,
    GreaterThan,
    Equals,
    NotEquals,
    // Delimiters
    Comma,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Token::*;
        match self {
            Eof => Ok(()),
            Identifier(id) => write!(f, "{id}"),
            Integer(int) => write!(f, "{int}"),
            Assign => write!(f, "="),
            Plus => write!(f, "+"),
            Minus => write!(f, "-"),
            Bang => write!(f, "!"),
            Asterisk => write!(f, "*"),
            Slash => write!(f, "/"),
            LessThan => write!(f, "<"),
            GreaterThan => write!(f, ">"),
            Equals => write!(f, "=="),
            NotEquals => write!(f, "!="),
            Comma => write!(f, ","),
            Semicolon => write!(f, ";"),
            LeftParen => write!(f, "("),
            RightParen => write!(f, ")"),
            LeftBrace => write!(f, "{{"),
            RightBrace => write!(f, "}}"),
            Function => write!(f, "fn"),
            Let => write!(f, "let"),
            True => write!(f, "true"),
            False => write!(f, "false"),
            If => write!(f, "if"),
            Else => write!(f, "else"),
            Return => write!(f, "return"),
        }
    }
}
