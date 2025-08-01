#include "token.h"
#include <iomanip>
#include <unordered_map>

static const std::unordered_map<TokenType, std::string> type_name_map{
    {TokenType::ILLEGAL, "ILLEGAL"},
    {TokenType::END_OF_FILE, "EOF"},
    {TokenType::IDENTIFIER, "IDENTIFIER"},
    {TokenType::INT, "INT"},
    {TokenType::ASSIGN, "ASSIGN"},
    {TokenType::PLUS, "PLUS"},
    {TokenType::MINUS, "MINUS"},
    {TokenType::BANG, "BANG"},
    {TokenType::ASTERISK, "ASTERISK"},
    {TokenType::SLASH, "SLASH"},
    {TokenType::LESS_THAN, "LESS_THAN"},
    {TokenType::GREATER_THAN, "GREATER_THAN"},
    {TokenType::EQUALS, "EQUALS"},
    {TokenType::NOT_EQUALS, "NOT_EQUALS"},
    {TokenType::COMMA, "COMMA"},
    {TokenType::SEMICOLON, "SEMICOLON"},
    {TokenType::LEFT_PAREN, "LEFT_PARENTHESIS"},
    {TokenType::RIGHT_PAREN, "RIGHT_PARENTHESIS"},
    {TokenType::LEFT_BRACE, "LEFT_BRACE"},
    {TokenType::RIGHT_BRACE, "RIGHT_BRACE"},
    {TokenType::FUNCTION, "FUNCTION"},
    {TokenType::LET, "LET"},
    {TokenType::TRUE, "TRUE"},
    {TokenType::FALSE, "FALSE"},
    {TokenType::IF, "IF"},
    {TokenType::ELSE, "ELSE"},
    {TokenType::RETURN, "RETURN"},
};

Token::Token(const TokenType type, std::string value)
    : type(type), value(std::move(value)) {
}

Token::Token(const TokenType type, const char ch)
    : Token{type, std::string{ch}} {
}

TokenType Token::getType() const {
    return type;
}

const std::string& Token::getValue() const {
    return value;
}

bool Token::operator==(const Token& other) const {
    return type == other.type && value == other.value;
}

// Token& Token::operator=(const Token& other) = default;

std::ostream& operator<<(std::ostream& out, const TokenType& type) {
    const auto entry = type_name_map.find(type);
    if (entry == type_name_map.end()) {
        throw std::domain_error{"Unknown token type"};
    }
    out << std::quoted(entry->second);
    return out;
}

std::ostream& operator<<(std::ostream& out, const Token& token) {
    out << "Token { type: " << token.getType() << ", ";
    out << "value: " << std::quoted(token.getValue()) << " }";
    return out;
}
