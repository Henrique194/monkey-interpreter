set(SOURCE_FILES
    lexer.cpp
    lexer.h
    main.cpp
    token.cpp
    token.h
)

if(WIN32)
    add_executable("monkey-interpreter" WIN32 ${SOURCE_FILES})
else()
    add_executable("monkey-interpreter" ${SOURCE_FILES})
endif()

target_include_directories("monkey-interpreter" PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

if(MSVC)
    set_target_properties("monkey-interpreter" PROPERTIES LINK_FLAGS "/MANIFEST:NO")
endif()
