#include "lexer.h"
#include "parser.h"
#include "repl.h"
#include <cstring>

static bool useRepl(const int argc, char** argv) {
    for (int i = 0; i < argc; i++) {
        if (std::strcmp(argv[i], "-repl") == 0) {
            return true;
        }
    }
    return false;
}

static bool testLetStatement(Statement& stmt, const std::string& name) {
    if (stmt.tokenLiteral() != "let") {
        std::cerr << "stmt.tokenLiteral not 'let'. " << std::endl;
        std::cerr << "got=" << stmt.tokenLiteral() << std::endl;
        return false;
    }
    if (typeid(stmt) != typeid(LetStatement)) {
        std::cerr << "stmt not LetStatement" << std::endl;
        std::cerr << "got=" << typeid(stmt).name() << std::endl;
        return false;
    }
    auto& letStmt = dynamic_cast<LetStatement&>(stmt);
    if (letStmt.name.value != name) {
        std::cerr << "letStmt.name.value not " << name << "." << std::endl;
        std::cerr << "got=" << letStmt.name.value << std::endl;
        return false;
    }
    if (letStmt.name.tokenLiteral() != name) {
        std::cerr << "letStmt.name.tokenLiteral() not " << name << "."
                  << std::endl;
        std::cerr << "got=" << letStmt.name.tokenLiteral() << std::endl;
        return false;
    }
    return true;
}

static void test(Parser& parser) {
    const std::vector<std::string> test{"x", "y", "foobar"};

    const auto program = parser.parseProgram();
    if (!program) {
        std::cerr << "parseProgram() returned nil" << std::endl;
        return;
    }
    if (program->statements.size() != test.size()) {
        std::cerr << "program->statements does not contain " << test.size()
                  << " statements.";
        std::cerr << "got=" << program->statements.size() << std::endl;
        return;
    }

    for (int i = 0; i < program->statements.size(); i++) {
        auto& stmt = program->statements[i];
        if (!testLetStatement(*stmt, test[i])) {
            return;
        }
    }
    std::cout << "tests - passed." << std::endl;
}

int main(int argc, char** argv) {
    if (useRepl(argc, argv)) {
        repl::run();
        return 0;
    }
    const std::filesystem::path filename = "main.mky";
    Lexer lexer{filename};
    Parser parser{std::move(lexer)};
    test(parser);
    return 0;
}
