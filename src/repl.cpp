#include "repl.h"
#include "lexer.h"

#ifdef _WIN32
#include <Windows.h>
#else
#include <unistd.h>
#include <pwd.h>
#endif


static const std::string PROMPT{">> "};


#ifdef _WIN32
static std::string getUserName() {
    constexpr int MAX_LEN = 100;
    char szBuffer[MAX_LEN];
    DWORD len = MAX_LEN;
    if (GetUserName(szBuffer, &len)) {
        return szBuffer;
    }
    return "user";
}
#else
static std::string getUserName() {
    uid_t userid = getuid();
    struct passwd* pwd = getpwuid(userid);
    return pwd->pw_name;
}
#endif

static void showGreeting() {
    std::cout << "Hello " << getUserName() << "! ";
    std::cout << "This is the Monkey programming language!" << std::endl;
    std::cout << "Feel free to type in commands" << std::endl;
}

void repl::run() {
    showGreeting();
    while (true) {
        std::cout << PROMPT;
        std::string input;
        if (!std::getline(std::cin, input) || input.empty()) {
            break;
        }
        Lexer lexer{std::move(input)};
        while (true) {
            const auto token = lexer.nextToken();
            if (token.type == TokenType::END_OF_FILE) {
                break;
            }
            std::cout << token << std::endl;
        }
    }
}
