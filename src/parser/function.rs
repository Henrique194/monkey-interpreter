use crate::parser::expression;
use crate::{
    ast::Expression,
    parser::{block, Parser},
    token::Token,
};

pub fn parse_literal<'a>(parser: &mut Parser<'a>) -> Result<Expression<'a>, String> {
    let parameters = parse_parameters(parser)?;
    let block = block::parse(parser)?;
    Ok(Expression::Function {
        parameters,
        block: Box::new(block),
    })
}

pub fn parse_call<'a>(
    parser: &mut Parser<'a>,
    function: Expression<'a>,
) -> Result<Expression<'a>, String> {
    let arguments = parse_call_arguments(parser)?;
    Ok(Expression::FunctionCall {
        function: Box::new(function),
        arguments,
    })
}

fn parse_parameters<'a>(parser: &mut Parser<'a>) -> Result<Vec<Expression<'a>>, String> {
    parser.expect_token(Token::LeftParen)?;
    parse_parenthesis(parser, |parser| {
        let id = parser.expect_identifier()?;
        Ok(Expression::Identifier(id))
    })
}

fn parse_call_arguments<'a>(parser: &mut Parser<'a>) -> Result<Vec<Expression<'a>>, String> {
    parse_parenthesis(parser, |parser| {
        parser.next_token();
        expression::parse(parser)
    })
}

fn parse_parenthesis<'a, F>(parser: &mut Parser<'a>, next: F) -> Result<Vec<Expression<'a>>, String>
where
    F: Fn(&mut Parser<'a>) -> Result<Expression<'a>, String>,
{
    if parser.peek_token == Token::RightParen {
        parser.next_token();
        return Ok(Vec::new());
    }
    let mut parameters = Vec::new();
    loop {
        let parm = next(parser)?;
        parameters.push(parm);
        if parser.peek_token != Token::Comma {
            break;
        }
        parser.next_token();
    }
    parser.expect_token(Token::RightParen)?;
    Ok(parameters)
}
