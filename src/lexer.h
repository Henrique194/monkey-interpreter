#ifndef LEXER_H
#define LEXER_H

#include "token.h"
#include <filesystem>
#include <fstream>
#include <functional>

class Lexer {
    std::string input;
    char ch;
    size_t position;
    size_t read_position;
    [[nodiscard]] char peekChar() const;
    char readChar();
    std::string readUntil(const std::function<bool(char)>& accept_char);
    std::optional<Token> readCharToken();
    std::optional<Token> readTwoCharToken();
    std::optional<Token> readWordToken();
    std::optional<Token> readNumberToken();
    void skipWhitespaces();

  public:
    explicit Lexer(const std::filesystem::path& path);
    Token nextToken();
};

#endif
