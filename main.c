#include <stdlib.h>
#include <stdio.h>

#include <parser_helper.h>

#include "interpreter.h"

extern int yyparse();
extern ast_t* prog;
extern FILE* yyin;
extern int yydebug;

int main(int argc, char** argv) {
  if(argc != 2) {
    printf("Expected filename, none found.\n");
    return 1;
  }

  yydebug = 0;
  yyin = fopen(argv[1], "r");
  yyparse();
  
  vmstate_t* state = malloc(sizeof(vmstate_t));
  state->vars = NULL;
  interpretloop(prog, state);
  
  return 0;
}
