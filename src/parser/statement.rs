use crate::{
    ast::Statement,
    parser::{expression, Parser},
    token::Token,
};

pub fn parse<'a>(parser: &mut Parser<'a>) -> Result<Statement<'a>, String> {
    let parse_result = match parser.current_token {
        Token::Let => parse_let_statement(parser),
        Token::Return => parse_return_statement(parser),
        _ => parse_expression_statement(parser),
    };
    if parser.peek_token == Token::Semicolon {
        parser.next_token();
    };
    parse_result
}

fn parse_let_statement<'a>(parser: &mut Parser<'a>) -> Result<Statement<'a>, String> {
    let identifier = parser.expect_identifier()?;
    parser.expect_token(Token::Assign)?;
    parser.next_token();
    let expression = expression::parse(parser)?;
    Ok(Statement::Let {
        identifier,
        expression,
    })
}

fn parse_return_statement<'a>(parser: &mut Parser<'a>) -> Result<Statement<'a>, String> {
    parser.next_token();
    let return_value = expression::parse(parser)?;
    Ok(Statement::Return(return_value))
}

fn parse_expression_statement<'a>(parser: &mut Parser<'a>) -> Result<Statement<'a>, String> {
    let expression = expression::parse(parser)?;
    Ok(Statement::Expression(expression))
}
