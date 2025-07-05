#include "lexer.h"

static void test(Lexer& lexer) {
    const std::vector<Token> test{
        {TokenType::LET, "let"},        {TokenType::IDENTIFIER, "five"},
        {TokenType::ASSIGN, "="},       {TokenType::INT, "5"},
        {TokenType::SEMICOLON, ";"},    {TokenType::LET, "let"},
        {TokenType::IDENTIFIER, "ten"}, {TokenType::ASSIGN, "="},
        {TokenType::INT, "10"},         {TokenType::SEMICOLON, ";"},
        {TokenType::LET, "let"},        {TokenType::IDENTIFIER, "add"},
        {TokenType::ASSIGN, "="},       {TokenType::FUNCTION, "fn"},
        {TokenType::LEFT_PAREN, "("},   {TokenType::IDENTIFIER, "x"},
        {TokenType::COMMA, ","},        {TokenType::IDENTIFIER, "y"},
        {TokenType::RIGHT_PAREN, ")"},  {TokenType::LEFT_BRACE, "{"},
        {TokenType::IDENTIFIER, "x"},   {TokenType::PLUS, "+"},
        {TokenType::IDENTIFIER, "y"},   {TokenType::SEMICOLON, ";"},
        {TokenType::RIGHT_BRACE, "}"},  {TokenType::SEMICOLON, ";"},
        {TokenType::LET, "let"},        {TokenType::IDENTIFIER, "result"},
        {TokenType::ASSIGN, "="},       {TokenType::IDENTIFIER, "add"},
        {TokenType::LEFT_PAREN, "("},   {TokenType::IDENTIFIER, "five"},
        {TokenType::COMMA, ","},        {TokenType::IDENTIFIER, "ten"},
        {TokenType::RIGHT_PAREN, ")"},  {TokenType::SEMICOLON, ";"},
        {TokenType::BANG, "!"},         {TokenType::MINUS, "-"},
        {TokenType::SLASH, "/"},        {TokenType::ASTERISK, "*"},
        {TokenType::INT, "5"},          {TokenType::SEMICOLON, ";"},
        {TokenType::INT, "5"},          {TokenType::LESS_THAN, "<"},
        {TokenType::INT, "10"},         {TokenType::GREATER_THAN, ">"},
        {TokenType::INT, "5"},          {TokenType::SEMICOLON, ";"},
        {TokenType::IF, "if"},          {TokenType::LEFT_PAREN, "("},
        {TokenType::INT, "5"},          {TokenType::LESS_THAN, "<"},
        {TokenType::INT, "10"},         {TokenType::RIGHT_PAREN, ")"},
        {TokenType::LEFT_BRACE, "{"},   {TokenType::RETURN, "return"},
        {TokenType::TRUE, "true"},      {TokenType::SEMICOLON, ";"},
        {TokenType::RIGHT_BRACE, "}"},  {TokenType::ELSE, "else"},
        {TokenType::LEFT_BRACE, "{"},   {TokenType::RETURN, "return"},
        {TokenType::FALSE, "false"},    {TokenType::SEMICOLON, ";"},
        {TokenType::RIGHT_BRACE, "}"},  {TokenType::INT, "10"},
        {TokenType::EQUALS, "=="},      {TokenType::INT, "10"},
        {TokenType::SEMICOLON, ";"},    {TokenType::INT, "10"},
        {TokenType::NOT_EQUALS, "!="},  {TokenType::INT, "9"},
        {TokenType::SEMICOLON, ";"},    {TokenType::END_OF_FILE, ""},
    };
    for (const auto& expectedToken : test) {
        const auto token = lexer.nextToken();
        if (expectedToken != token) {
            std::cerr << "tests - token value wrong." << std::endl;
            std::cerr << "expected = " << expectedToken << std::endl;
            std::cerr << "received = " << token << std::endl;
            return;
        }
        std::cout << token << std::endl;
    }
    std::cout << "tests - passed." << std::endl;
}

int main() {
    const auto filename = "main.mky";
    Lexer lexer{filename};
    test(lexer);
    return 0;
}
