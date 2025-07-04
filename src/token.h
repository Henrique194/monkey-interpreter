#ifndef TOKEN_H
#define TOKEN_H

#include <iostream>
#include <string>

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

struct Token {
    const TokenType type;
    const std::string value;
    Token(TokenType type, std::string value);
    Token(TokenType type, char ch);
    bool operator==(const Token& other) const;
};

std::ostream& operator<<(std::ostream& out, const TokenType& type);
std::ostream& operator<<(std::ostream& out, const Token& token);

#endif
