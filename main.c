#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <ctype.h>

#include "common.h"
#include "lexer.h"
#include "parser.h"

/*
ast_token_t* parse(tokenstream_t ts) {
  ast_token_t* head = malloc(sizeof(ast_token_t));
  ast_token_t* cur = head;
  
  while(ts->tok->type != EOF) {
    switch(ts->tok->type) {
    INTEGER:
      cur->type = INTEGER;
      cur->iVal = ts->tok->iVal;
      break;
    PLUS:
      cur->type = PLUS;
*/

result_t* interpret(ast_t* t) {
  result_t* res = malloc(sizeof(result_t));
  
  switch(t->type) {
  case AST_INT:
    res->type = RES_INT;
    res->item.iVal = t->item.iVal;

    freeast(t);
    return res;
  case AST_BINOP:
    {
      result_t* lt = interpret(t->item.binop.left);

      if(lt->type == RES_ERROR) {
	freeast(t);
	free(res);
	return lt;
      }
      
      result_t* rt = interpret(t->item.binop.right);

      if(rt->type == RES_ERROR) {
	freeast(t);
	free(res);
	free(lt);
	return rt;
      }
      
      int l = lt->item.iVal;
      int r = rt->item.iVal;

      free(lt);
      free(rt);

      t->item.binop.left = NULL;
      t->item.binop.right = NULL;

      res->type = RES_INT;

      switch(t->item.binop.type) {
      case AST_BINOP_ADD:
        res->item.iVal = l + r;
	break;
      case AST_BINOP_SUB:
	res->item.iVal = l - r;
	break;
      case AST_BINOP_MUL:
	res->item.iVal = l * r;
	break;
      case AST_BINOP_DIV:
	res->item.iVal = l / r;
	break;
      }

      freeast(t);
      return res;
    }
  case AST_ERROR:
    res->type = RES_ERROR;
    res->item.error = t->item.error;

    freeast(t);
    return res;
  }
}

int main(void) {
  char line[80];
  int done = 0;

  while(done == 0) {
    printf("calc> ");
    fgets(line, 80, stdin);

    tokenstream_t* ts = lexfullline(line);
    ast_t* ast = parse(ts);
    result_t* res = interpret(ast);

    switch(res->type) {
    case RES_INT:
      printf("%d\n", res->item.iVal);
      break;
    case RES_ERROR:
      printf("%s\n", res->item.error);
      break;
    }
    
    free(res);
  }
  
  return 0;
}
