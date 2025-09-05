use crate::parser::function;
use crate::{
    ast::Expression,
    parser::{
        Parser,
        expression::{Precedence, parse_precedence},
    },
    token::Token,
};

pub fn parse<'a>(
    parser: &mut Parser<'a>,
    mut lhs: Expression<'a>,
    precedence: Precedence,
) -> Result<Expression<'a>, String> {
    while parser.peek_token != Token::Semicolon {
        let infix = parser.peek_token;
        if get_precedence(infix) <= precedence {
            break;
        }
        parser.next_token();
        lhs = infix_rule(parser, lhs)?;
    }
    Ok(lhs)
}

fn get_precedence(token: Token) -> Precedence {
    use Token::*;
    match token {
        Equals | NotEquals => Precedence::Equality,
        LessThan | GreaterThan => Precedence::Comparison,
        Plus | Minus => Precedence::Term,
        Slash | Asterisk => Precedence::Factor,
        LeftParen => Precedence::Call,
        _ => Precedence::None,
    }
}

fn infix_rule<'a>(parser: &mut Parser<'a>, lhs: Expression<'a>) -> Result<Expression<'a>, String> {
    use Token::*;
    match parser.current_token {
        Equals | NotEquals | LessThan | GreaterThan | Plus | Minus | Slash | Asterisk => {
            parse_binary_op(parser, lhs)
        }
        LeftParen => function::parse_call(parser, lhs),
        _ => unreachable!("No infix rule for expression: {lhs}"),
    }
}

fn parse_binary_op<'a>(
    parser: &mut Parser<'a>,
    lhs: Expression<'a>,
) -> Result<Expression<'a>, String> {
    let operator = parser.current_token;
    let precedence = get_precedence(operator);
    parser.next_token();
    let rhs = parse_precedence(parser, precedence)?;
    Ok(Expression::Infix {
        operator,
        left: Box::new(lhs),
        right: Box::new(rhs),
    })
}
