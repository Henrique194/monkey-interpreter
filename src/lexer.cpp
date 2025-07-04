#include "lexer.h"
#include <unordered_map>

namespace fs = std::filesystem;

static const std::unordered_map<char, TokenType> char_tokens{
    {'\0', TokenType::END_OF_FILE}, {'=', TokenType::ASSIGN},
    {'+', TokenType::PLUS},         {'-', TokenType::MINUS},
    {'!', TokenType::BANG},         {'*', TokenType::ASTERISK},
    {'/', TokenType::SLASH},        {'<', TokenType::LESS_THAN},
    {'>', TokenType::GREATER_THAN}, {',', TokenType::COMMA},
    {';', TokenType::SEMICOLON},    {'(', TokenType::LEFT_PAREN},
    {')', TokenType::RIGHT_PAREN},  {'{', TokenType::LEFT_BRACE},
    {'}', TokenType::RIGHT_BRACE},
};

static const std::unordered_map<std::string, TokenType> two_char_tokens{
    {"==", TokenType::EQUALS},
    {"!=", TokenType::NOT_EQUALS},
};

static const std::unordered_map<std::string, TokenType> keywords{
    {"fn", TokenType::FUNCTION},   {"let", TokenType::LET},
    {"true", TokenType::TRUE},     {"false", TokenType::FALSE},
    {"if", TokenType::IF},         {"else", TokenType::ELSE},
    {"return", TokenType::RETURN},
};

static bool isLetter(const char c) {
    return ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z') || c == '_';
}

static bool isNumber(const char c) {
    return std::isdigit(c);
}

static std::string readFile(const fs::path& path) {
    std::ifstream file;
    file.exceptions(std::ifstream::failbit | std::ifstream::badbit);
    file.open(path);

    const auto file_size = static_cast<std::streamsize>(fs::file_size(path));
    std::string file_content(file_size, '\0');
    file.read(file_content.data(), file_size);

    return file_content;
}

Lexer::Lexer(const fs::path& path)
    : input(readFile(path)), ch('\0'), position(0), read_position(0) {
    readChar();
}

char Lexer::peekChar() const {
    if (read_position >= input.length()) {
        return '\0';
    }
    return input[read_position];
}

char Lexer::readChar() {
    ch = peekChar();
    position = read_position;
    read_position++;
    return ch;
}

std::string Lexer::readUntil(const std::function<bool(char)>& accept_char) {
    const auto initial_pos = position;
    while (accept_char(ch)) {
        readChar();
    }
    const auto length = position - initial_pos;
    return input.substr(initial_pos, length);
}

std::optional<Token> Lexer::readCharToken() {
    const auto entry = char_tokens.find(ch);
    if (entry == char_tokens.end()) {
        return std::nullopt;
    }
    const auto type = entry->second;
    std::string value{};
    if (entry->first != '\0') {
        value = std::string{entry->first};
    }
    // Advance one character.
    readChar();
    return Token{type, value};
}

std::optional<Token> Lexer::readTwoCharToken() {
    const char c1 = ch;
    const char c2 = peekChar();
    const std::string value{c1, c2};
    const auto entry = two_char_tokens.find(value);
    if (entry == two_char_tokens.end()) {
        return std::nullopt;
    }
    // Advance two characters.
    readChar();
    readChar();
    return Token{entry->second, value};
}

std::optional<Token> Lexer::readWordToken() {
    const auto word = readUntil(isLetter);
    if (word.empty()) {
        return std::nullopt;
    }
    TokenType type;
    if (const auto entry = keywords.find(word); entry != keywords.end()) {
        type = entry->second;
    } else {
        type = TokenType::IDENTIFIER;
    }
    return Token{type, word};
}

std::optional<Token> Lexer::readNumberToken() {
    const auto num = readUntil(isNumber);
    if (num.empty()) {
        return std::nullopt;
    }
    return Token{TokenType::INT, num};
}

void Lexer::skipWhitespaces() {
    while (std::isspace(ch)) {
        readChar();
    }
}

Token Lexer::nextToken() {
    skipWhitespaces();
    if (const auto token = readTwoCharToken()) {
        return *token;
    }
    if (const auto token = readCharToken()) {
        return *token;
    }
    if (const auto token = readWordToken()) {
        return *token;
    }
    if (const auto token = readNumberToken()) {
        return *token;
    }
    return Token{TokenType::ILLEGAL, ch};
}
