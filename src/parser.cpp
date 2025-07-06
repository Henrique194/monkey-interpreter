#include "parser.h"

Identifier::Identifier(Token&& token, std::string&& value)
    : token(std::move(token)), value(std::move(value)) {
}

LetStatement::LetStatement(Token&& token, Identifier&& id)
    : token(std::move(token)), name(std::move(id)) {
}


void Identifier::expressionNode() {
}

std::string Identifier::tokenLiteral() {
    return token.getValue();
}

void LetStatement::statementNode() {
}

std::string LetStatement::tokenLiteral() {
    return token.getValue();
}

std::string Program::tokenLiteral() {
    if (statements.empty()) {
        return "";
    }
    return statements[0]->tokenLiteral();
}

Parser::Parser(Lexer&& lexer)
    : lexer(std::move(lexer)), current_token{TokenType::END_OF_FILE, '\0'},
      peek_token{TokenType::END_OF_FILE, '\0'} {
    nextToken();
    nextToken();
}

void Parser::nextToken() {
    current_token = peek_token;
    peek_token = lexer.nextToken();
}

bool Parser::currentTokenIs(const TokenType type) const {
    return current_token.getType() == type;
}

bool Parser::peekTokenIs(const TokenType type) const {
    return peek_token.getType() == type;
}

bool Parser::expectPeek(const TokenType type) {
    if (peekTokenIs(type)) {
        nextToken();
        return true;
    }
    return false;
}

std::optional<LetStatement> Parser::parseLetStatement() {
    auto let_token = current_token;
    if (!expectPeek(TokenType::IDENTIFIER)) {
        return std::nullopt;
    }
    auto id_token = current_token;
    if (!expectPeek(TokenType::ASSIGN)) {
        return std::nullopt;
    }
    while (!currentTokenIs(TokenType::SEMICOLON)) {
        nextToken();
    }
    std::string literal = id_token.getValue();
    Identifier id{std::move(id_token), std::move(literal)};
    return LetStatement{std::move(let_token), std::move(id)};
}

std::optional<std::unique_ptr<Statement>> Parser::parseStatement() {
    switch (current_token.getType()) {
        case TokenType::LET:
            if (auto stmt = parseLetStatement()) {
                return std::make_unique<LetStatement>(*stmt);
            }
            return std::nullopt;
        default:
            return std::nullopt;
    }
}


std::optional<Program> Parser::parseProgram() {
    Program program{};
    while (current_token.getType() != TokenType::END_OF_FILE) {
        if (auto stmt = parseStatement()) {
            program.statements.push_back(std::move(*stmt));
        }
        nextToken();
    }
    return program;
}
