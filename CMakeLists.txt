cmake_minimum_required(VERSION 2.8.8)
project(rustint)

find_package(BISON)
find_package(FLEX)

BISON_TARGET(RustParser parser.y ${CMAKE_CURRENT_BINARY_DIR}/parser.c)
FLEX_TARGET(RustScanner lexer.l ${CMAKE_CURRENT_BINARY_DIR}/lexer.c)
ADD_FLEX_BISON_DEPENDENCY(RustScanner RustParser)

include_directories(include)

add_executable(rustint
	main.c
	interpreter.c
	parser_helper.c
	${BISON_RustParser_OUTPUTS}
	${FLEX_RustScanner_OUTPUTS})
INSTALL(TARGETS rustint RUNTIME DESTINATION /usr/bin/)
