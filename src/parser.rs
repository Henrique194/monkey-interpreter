use crate::{ast::Program, lexer::Lexer, token::Token};

mod block;
mod expression;
mod function;
mod statement;

#[cfg(test)]
mod tests;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token<'a>,
    peek_token: Token<'a>,
    panic_mode: bool,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut parser = Self {
            lexer: Lexer::new(input),
            current_token: Token::Eof,
            peek_token: Token::Eof,
            panic_mode: false,
            errors: Vec::new(),
        };
        // Initialize current and peek token.
        parser.next_token();
        parser.next_token();
        parser
    }

    pub fn parse(&mut self) -> Program<'a> {
        let mut program = Program::new();
        while self.current_token != Token::Eof {
            match statement::parse(self) {
                Ok(statement) => {
                    program.add_statement(statement);
                    self.panic_mode = false;
                }
                Err(error) => {
                    if !self.panic_mode {
                        self.add_error(error);
                        self.panic_mode = true;
                    }
                }
            };
            self.next_token();
        }
        program
    }

    pub fn errors(&self) -> &[String] {
        &self.errors
    }

    fn expect_identifier(&mut self) -> Result<&'a str, String> {
        let token = self.peek_token;
        match token {
            Token::Identifier(id) => {
                self.next_token();
                Ok(id)
            }
            _ => Err(format!("expected identifier, received {token}")),
        }
    }

    fn expect_token(&mut self, token: Token<'a>) -> Result<(), String> {
        if self.peek_token == token {
            self.next_token();
            return Ok(());
        }
        Err(format!("expected {}, received {token}", self.peek_token))
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token;
        self.peek_token = self.lexer.next_token();
    }

    fn add_error(&mut self, error: String) {
        let line = self.lexer.line();
        let error = format!("error at line {line}: {error}");
        self.errors.push(error);
    }
}
