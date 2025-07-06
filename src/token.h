#ifndef TOKEN_H
#define TOKEN_H

#include <iostream>

enum class TokenType {
    ILLEGAL,
    END_OF_FILE,
    // Identifiers + literals
    IDENTIFIER,
    INT,
    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LESS_THAN,
    GREATER_THAN,
    EQUALS,
    NOT_EQUALS,
    // Delimiters
    COMMA,
    SEMICOLON,
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    // Keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
};

class Token {
    TokenType type;
    std::string value;

  public:
    Token(TokenType type, std::string value);
    Token(TokenType type, char ch);
    TokenType getType() const;
    const std::string& getValue() const;
    bool operator==(const Token& other) const;
    // Token& Token::operator=(const Token& other);
};

std::ostream& operator<<(std::ostream& out, const TokenType& type);
std::ostream& operator<<(std::ostream& out, const Token& token);

#endif
