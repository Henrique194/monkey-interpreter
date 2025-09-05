use crate::parser::Parser;
use std::error::Error;
use std::fs;

mod ast;
mod lexer;
mod parser;
mod token;

fn main() -> Result<(), Box<dyn Error>> {
    let code = fs::read_to_string("assets/main.mky")?;
    let mut parser = Parser::new(&code);
    let program = parser.parse();
    println!("{program}");
    for err in parser.errors() {
        println!("{err}");
    }
    Ok(())
}
