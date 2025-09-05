use crate::token::Token;

#[cfg(test)]
mod tests;

pub struct Lexer<'a> {
    input: &'a str,
    line: usize,
    ch: char,
    current_position: usize,
    read_position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Self {
            input,
            line: 1,
            ch: '\0',
            current_position: 0,
            read_position: 0,
        };
        lexer.read_char();
        lexer
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn next_token(&mut self) -> Token<'a> {
        self.skip_whitespace();
        if self.ch == '\0' {
            return Token::Eof;
        }
        self.lex_fixed_length_token(2)
            .or_else(|| self.lex_fixed_length_token(1))
            .or_else(|| self.lex_word())
            .or_else(|| self.lex_number())
            .expect("Lexer encountered illegal Token")
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            if self.ch == '\n' {
                self.line += 1;
            }
            self.read_char();
        }
    }

    fn lex_fixed_length_token(&mut self, length: usize) -> Option<Token<'a>> {
        let token_symbol = self.peek(length);
        let token = get_token(token_symbol)?;
        for _ in 0..length {
            self.read_char();
        }
        Some(token)
    }

    fn lex_number(&mut self) -> Option<Token<'a>> {
        let number = self.read_while(char::is_ascii_digit)?;
        Some(Token::Integer(number))
    }

    fn lex_word(&mut self) -> Option<Token<'a>> {
        let word = self.read_while(is_letter)?;
        if let Some(keyword) = get_token(&word) {
            return Some(keyword);
        }
        Some(Token::Identifier(word))
    }

    fn read_char(&mut self) {
        let mut chars = self.input.chars();
        if let Some(new_char) = chars.nth(self.read_position) {
            self.ch = new_char;
            self.current_position = self.read_position;
            self.read_position += 1;
            return;
        }
        self.ch = '\0';
        self.current_position = self.read_position;
    }

    pub fn read_while<F>(&mut self, accept_char: F) -> Option<&'a str>
    where
        F: Fn(&char) -> bool,
    {
        let start_pos = self.current_position;
        while accept_char(&self.ch) {
            self.read_char();
        }
        let final_pos = self.current_position;
        if start_pos == final_pos {
            return None;
        }
        Some(&self.input[start_pos..final_pos])
    }

    fn peek(&self, n: usize) -> &'a str {
        let start_pos = self.current_position;
        let final_pos = std::cmp::min(start_pos + n, self.input.len());
        &self.input[start_pos..final_pos]
    }
}

fn is_letter(ch: &char) -> bool {
    ch.is_ascii_alphabetic() || *ch == '_'
}

fn get_token(token_symbol: &str) -> Option<Token<'static>> {
    let token = match token_symbol {
        "=" => Token::Assign,
        "+" => Token::Plus,
        "-" => Token::Minus,
        "!" => Token::Bang,
        "*" => Token::Asterisk,
        "/" => Token::Slash,
        "<" => Token::LessThan,
        ">" => Token::GreaterThan,
        "==" => Token::Equals,
        "!=" => Token::NotEquals,
        "," => Token::Comma,
        ";" => Token::Semicolon,
        "(" => Token::LeftParen,
        ")" => Token::RightParen,
        "{" => Token::LeftBrace,
        "}" => Token::RightBrace,
        "let" => Token::Let,
        "fn" => Token::Function,
        "if" => Token::If,
        "else" => Token::Else,
        "true" => Token::True,
        "false" => Token::False,
        "return" => Token::Return,
        _ => return None,
    };
    Some(token)
}
