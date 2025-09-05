use crate::token::Token;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

pub struct Program<'a> {
    statements: Vec<Statement<'a>>,
}

impl<'a> Program<'a> {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }

    pub fn add_statement(&mut self, statement: Statement<'a>) {
        self.statements.push(statement);
    }
}

impl Display for Program<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for statement in &self.statements {
            writeln!(f, "{statement}")?;
        }
        Ok(())
    }
}

pub enum Statement<'a> {
    Let {
        identifier: &'a str,
        expression: Expression<'a>,
    },
    Return(Expression<'a>),
    Expression(Expression<'a>),
}

impl Display for Statement<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Let {
                identifier,
                expression,
            } => write!(f, "let {identifier} = {expression};"),
            Statement::Return(return_value) => write!(f, "return {return_value};"),
            Statement::Expression(expression) => write!(f, "{expression};"),
        }
    }
}

pub enum Expression<'a> {
    Integer(i64),
    Boolean(bool),
    Identifier(&'a str),
    Unary {
        operator: Token<'a>,
        expression: Box<Expression<'a>>,
    },
    Infix {
        operator: Token<'a>,
        left: Box<Expression<'a>>,
        right: Box<Expression<'a>>,
    },
    If {
        condition: Box<Expression<'a>>,
        consequence: Box<Expression<'a>>,
        alternative: Box<Option<Expression<'a>>>,
    },
    Function {
        parameters: Vec<Expression<'a>>,
        block: Box<Expression<'a>>,
    },
    FunctionCall {
        function: Box<Expression<'a>>,
        arguments: Vec<Expression<'a>>,
    },
    Block(Vec<Statement<'a>>),
}

impl Display for Expression<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Expression::*;
        match self {
            Integer(int) => write!(f, "{int}"),
            Boolean(boolean) => write!(f, "{boolean}"),
            Identifier(id) => write!(f, "{id}"),
            Unary {
                operator,
                expression,
            } => write!(f, "{operator}({expression})"),
            Infix {
                operator,
                left,
                right,
            } => write!(f, "({left} {operator} {right})"),
            If {
                condition,
                consequence,
                alternative,
            } => {
                write!(f, "if ({condition}) {consequence}")?;
                if let Some(alternative) = alternative.deref() {
                    write!(f, " else {alternative}")?;
                }
                Ok(())
            }
            FunctionCall {
                function,
                arguments,
            } => {
                if matches!(**function, Identifier(_)) {
                    write!(f, "{function}(")?;
                } else {
                    write!(f, "({function})(")?;
                }
                write_args(f, arguments)?;
                write!(f, ")")
            }
            Function { parameters, block } => {
                write!(f, "fn(")?;
                write_args(f, parameters)?;
                write!(f, ") {block}")
            }
            Block(statements) => {
                writeln!(f, "{{")?;
                for stmt in statements {
                    writeln!(f, "{stmt}")?;
                }
                write!(f, "}}")
            }
        }
    }
}

fn write_args(f: &mut Formatter<'_>, args: &[Expression<'_>]) -> std::fmt::Result {
    for (i, arg) in args.iter().enumerate() {
        if i == args.len() - 1 {
            write!(f, "{arg}")?;
        } else {
            write!(f, "{arg}, ")?;
        }
    }
    Ok(())
}
