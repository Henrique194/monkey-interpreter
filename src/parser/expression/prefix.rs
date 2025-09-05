use crate::{
    ast::Expression,
    parser::{block, expression, expression::Precedence, function, Parser},
    token::Token,
};

pub fn parse<'a>(parser: &mut Parser<'a>) -> Result<Expression<'a>, String> {
    use Token::*;
    let token = parser.current_token;
    match token {
        True => Ok(Expression::Boolean(true)),
        False => Ok(Expression::Boolean(false)),
        Integer(int) => parse_integer(int),
        Identifier(id) => Ok(Expression::Identifier(id)),
        LeftParen => parse_grouped_expression(parser),
        Function => function::parse_literal(parser),
        Bang | Minus => parse_unary(parser),
        If => parse_if(parser),
        _ => Err(format!("no prefix parse function for \"{token}\" found")),
    }
}

fn parse_integer<'a>(int: &str) -> Result<Expression<'a>, String> {
    let Ok(int) = int.parse::<i64>() else {
        return Err(format!("could not parse {int} as integer"));
    };
    Ok(Expression::Integer(int))
}

fn parse_grouped_expression<'a>(parser: &mut Parser<'a>) -> Result<Expression<'a>, String> {
    parser.next_token();
    let expression = expression::parse(parser)?;
    parser.expect_token(Token::RightParen)?;
    Ok(expression)
}

fn parse_unary<'a>(parser: &mut Parser<'a>) -> Result<Expression<'a>, String> {
    let precedence = Precedence::Unary;
    let operator = parser.current_token;
    parser.next_token();
    let expression = expression::parse_precedence(parser, precedence)?;
    Ok(Expression::Unary {
        operator,
        expression: Box::new(expression),
    })
}

fn parse_if<'a>(parser: &mut Parser<'a>) -> Result<Expression<'a>, String> {
    parser.expect_token(Token::LeftParen)?;
    parser.next_token();
    let condition = expression::parse(parser)?;
    parser.expect_token(Token::RightParen)?;

    let consequence = block::parse(parser)?;
    let alternative = if parser.peek_token == Token::Else {
        parser.next_token();
        Some(block::parse(parser)?)
    } else {
        None
    };

    Ok(Expression::If {
        condition: Box::new(condition),
        consequence: Box::new(consequence),
        alternative: Box::new(alternative),
    })
}
