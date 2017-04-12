#include <stdlib.h>
#include <stdio.h>

#include <parser_helper.h>

#include "interpreter.h"

extern int yyparse();
extern ast_stmt_t* prog;
extern FILE* yyin;
extern int yydebug;

struct ast_identifier {

};

extern void execute_rust();
extern struct ast_identifier create_identifier(const char*);
extern void print_identifier(struct ast_identifier);

int main(int argc, char** argv) {
  if(argc != 2) {
    printf("Expected filename, none found.\n");
    return 1;
  }

  execute_rust();
  
  print_identifier(create_identifier("Test"));

  yydebug = 0;
  yyin = fopen(argv[1], "r");
  yyparse();
  
  scope_t* global = malloc(sizeof(scope_t));
  global->parent = NULL;
  global->vars = NULL;
  
  interpretloop(prog, global);
  
  return 0;
}
