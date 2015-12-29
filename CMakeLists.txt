cmake_minimum_required(VERSION 2.6)
project(rustint)

if(UNIX)
	if(CMAKE_COMPILER_IS_GNUCC)
		set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -std=c99 -Wall -Wextra -pedantic -Wshadow -Wpointer-arith -Wcast-align -Wwrite-strings -Wmissing-prototypes -Wmissing-declarations -Wredundant-decls -Wnested-externs -Winline -Wno-long-long -Wuninitialized -Wconversion -Wstrict-prototypes")
	endif(CMAKE_COMPILER_IS_GNUCC)
endif(UNIX)

add_executable(rustint main.c lexer.c parser.c)
INSTALL(TARGETS rustint RUNTIME DESTINATION /usr/bin/)
