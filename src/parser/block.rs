use crate::ast::Expression;
use crate::parser::{statement, Parser};
use crate::token::Token;

pub fn parse<'a>(parser: &mut Parser<'a>) -> Result<Expression<'a>, String> {
    parser.expect_token(Token::LeftBrace)?;
    let mut statements = Vec::new();
    while parser.peek_token != Token::RightBrace {
        parser.next_token();
        let stmt = statement::parse(parser)?;
        statements.push(stmt);
    }
    parser.next_token();
    Ok(Expression::Block(statements))
}
