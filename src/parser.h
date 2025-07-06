#ifndef PARSER_H
#define PARSER_H

#include "lexer.h"
#include <memory>
#include <string>
#include <vector>

class Node {
  public:
    virtual ~Node() = default;
    virtual std::string tokenLiteral() = 0;
};

class Statement : public Node {
  public:
    virtual void statementNode() = 0;
};

class Expression : public Node {
  public:
    virtual void expressionNode() = 0;
};

class Program : public Node {
  public:
    std::vector<std::unique_ptr<Statement>> statements{};
    std::string tokenLiteral() override;
};

class Identifier : public Expression {
  public:
    Token token;
    std::string value;
    Identifier(Token&& token, std::string&& value);
    void expressionNode() override;
    std::string tokenLiteral() override;
};

class LetStatement : public Statement {
  public:
    Token token;
    Identifier name;
    LetStatement(Token&& token, Identifier&& id);
    void statementNode() override;
    std::string tokenLiteral() override;
};

class Parser {
    Lexer lexer;
    Token current_token;
    Token peek_token;

  public:
    explicit Parser(Lexer&& lexer);
    void nextToken();
    bool currentTokenIs(TokenType type) const;
    bool peekTokenIs(TokenType type) const;
    bool expectPeek(TokenType type);
    std::optional<Program> parseProgram();
    std::optional<std::unique_ptr<Statement>> parseStatement();
    std::optional<LetStatement> parseLetStatement();
};

#endif