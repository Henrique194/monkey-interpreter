use crate::ast::Expression;
use crate::parser::Parser;

mod infix;
mod prefix;

pub fn parse<'a>(parser: &mut Parser<'a>) -> Result<Expression<'a>, String> {
    parse_precedence(parser, Precedence::None)
}

fn parse_precedence<'a>(
    parser: &mut Parser<'a>,
    precedence: Precedence,
) -> Result<Expression<'a>, String> {
    let lhs = prefix::parse(parser)?;
    infix::parse(parser, lhs, precedence)
}

#[derive(PartialEq, Eq, PartialOrd, Copy, Clone)]
enum Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < >
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // ()
}
