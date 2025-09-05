use super::*;

fn test_lexer<const N: usize>(input: &str, tokens: [Token; N]) {
    let mut lexer = Lexer::new(input);
    for expected_token in tokens {
        let received_token = lexer.next_token();
        assert_eq!(received_token, expected_token);
    }
}

#[test]
fn operations() {
    let input = "=+-!*/<>==!=";
    let tokens = [
        Token::Assign,
        Token::Plus,
        Token::Minus,
        Token::Bang,
        Token::Asterisk,
        Token::Slash,
        Token::LessThan,
        Token::GreaterThan,
        Token::Equals,
        Token::NotEquals,
        Token::Eof,
    ];
    test_lexer(input, tokens);
}

#[test]
fn variable_declaration() {
    let input = r#"
        let five = 5;
        let ten = 10;
        let one_hundred = 100
    "#;
    let tokens = [
        Token::Let,
        Token::Identifier("five"),
        Token::Assign,
        Token::Integer("5"),
        Token::Semicolon,
        Token::Let,
        Token::Identifier("ten"),
        Token::Assign,
        Token::Integer("10"),
        Token::Semicolon,
        Token::Let,
        Token::Identifier("one_hundred"),
        Token::Assign,
        Token::Integer("100"),
        Token::Eof,
    ];
    test_lexer(input, tokens);
}

#[test]
fn closure_declaration() {
    let input = r#"
        let add = fn(x, y) {
            x + y;
        };
    "#;
    let tokens = [
        Token::Let,
        Token::Identifier("add"),
        Token::Assign,
        Token::Function,
        Token::LeftParen,
        Token::Identifier("x"),
        Token::Comma,
        Token::Identifier("y"),
        Token::RightParen,
        Token::LeftBrace,
        Token::Identifier("x"),
        Token::Plus,
        Token::Identifier("y"),
        Token::Semicolon,
        Token::RightBrace,
        Token::Semicolon,
        Token::Eof,
    ];
    test_lexer(input, tokens);
}

#[test]
fn comparison() {
    let input = r#"
        10 == 10;
        10 != 9;
        5 < 10;
        8 > 2;
    "#;
    let tokens = [
        Token::Integer("10"),
        Token::Equals,
        Token::Integer("10"),
        Token::Semicolon,
        Token::Integer("10"),
        Token::NotEquals,
        Token::Integer("9"),
        Token::Semicolon,
        Token::Integer("5"),
        Token::LessThan,
        Token::Integer("10"),
        Token::Semicolon,
        Token::Integer("8"),
        Token::GreaterThan,
        Token::Integer("2"),
        Token::Semicolon,
        Token::Eof,
    ];
    test_lexer(input, tokens);
}

#[test]
fn conditional() {
    let input = r#"
        if (a < b) {
            return true;
        } else {
            return false;
        }
    "#;
    let tokens = [
        Token::If,
        Token::LeftParen,
        Token::Identifier("a"),
        Token::LessThan,
        Token::Identifier("b"),
        Token::RightParen,
        Token::LeftBrace,
        Token::Return,
        Token::True,
        Token::Semicolon,
        Token::RightBrace,
        Token::Else,
        Token::LeftBrace,
        Token::Return,
        Token::False,
        Token::Semicolon,
        Token::RightBrace,
        Token::Eof,
    ];
    test_lexer(input, tokens);
}
