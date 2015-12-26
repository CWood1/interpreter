all:
	gcc main.c lexer.c parser.c -o calc
debug:
	gcc -O0 -g main.c lexer.c parser.c -o calc
