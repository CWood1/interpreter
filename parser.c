#include <stdlib.h>

#include "common.h"
#include "lexer.h"
#include "parser.h"

int isinteger(token_t* t) {
  if(t->type == INTEGER)
    return 1;
  else
    return 0;
}

int isend(token_t* t) {
  if(t->type == END)
    return 1;
  else
    return 0;
}

int isadditiveoperation(token_t* t) {
  if(t->type == PLUS)
    return 1;
  else if(t->type == MINUS)
    return 1;
  else
    return 0;
}

int ismultiplicativeoperation(token_t* t) {
  if(t->type == MULTIPLY)
    return 1;
  else if(t->type == DIVIDE)
    return 1;
  else if(t->type == MODULO)
    return 1;
  else
    return 0;
}

int islparen(token_t* t) {
  if(t->type == LPAREN)
    return 1;
  else
    return 0;
}

int isrparen(token_t* t) {
  if(t->type == RPAREN)
    return 1;
  else
    return 0;
}

ast_t* term(tokenstream_t* ts, token_t* t) {
  ast_t* res;

  if(isinteger(t)) {
    res = malloc(sizeof(ast_t));
    res->type = AST_INT;
    res->item.iVal = t->item.iVal;

    ts->head = t->next;
    free(t);

    return res;
  } else if(islparen(t)) {
    ts->head = t->next;
    free(t);
    t = ts->head;

    res = expr(ts, t);
    
    if(res->type == AST_ERROR)
      return res;
      
    t = ts->head;

    if(isrparen(t) == 0) {
      freetokenstream(ts);
      freeast(res);

      res = malloc(sizeof(ast_t));
      
      res->type = AST_ERROR;
      res->item.error = "Syntax error - unclosed parenthesis\n";

      return res;
    }

    ts->head = t->next;
    free(t);

    return res;
  } else {
    res = malloc(sizeof(ast_t));
    res->type = AST_ERROR;
    res->item.error = "Syntax error - unexpected token\n";

    freetokenstream(ts);

    return res;
  }
}

ast_t* factor(tokenstream_t* ts, token_t* t) {
  ast_t* res = term(ts, t);
  ast_t* nextres;

  if(res->type == AST_ERROR)
    return res;

  t = ts->head;

  while(ismultiplicativeoperation(t)) {
    switch(t->type) {
    case MULTIPLY:
      ts->head = t->next;
      free(t);
      t = ts->head;

      nextres = malloc(sizeof(ast_t));
      nextres->type = AST_BINOP;
      nextres->item.binop.type = AST_BINOP_MUL;
      nextres->item.binop.left = res;

      res = nextres;
      nextres = factor(ts, t);
      
      if(nextres->type == AST_ERROR) {
	freeast(res);
	return nextres;
      }
      
      t = ts->head;
      res->item.binop.right = nextres;

      break;
    case DIVIDE:
      ts->head = t->next;
      free(t);
      t = ts->head;

      nextres = malloc(sizeof(ast_t));
      nextres->type = AST_BINOP;
      nextres->item.binop.type = AST_BINOP_DIV;
      nextres->item.binop.left = res;

      res = nextres;
      nextres = factor(ts, t);
      
      if(nextres->type == AST_ERROR) {
	freeast(res);
	return nextres;
      }
      
      t = ts->head;
      res->item.binop.right = nextres;

      break;
    case MODULO:
      ts->head = t->next;
      free(t);
      t = ts->head;

      nextres = malloc(sizeof(ast_t));
      nextres->type = AST_BINOP;
      nextres->item.binop.type = AST_BINOP_MOD;
      nextres->item.binop.left = res;

      res = nextres;
      nextres = factor(ts, t);
      
      if(nextres->type == AST_ERROR) {
	freeast(res);
	return nextres;
      }
      
      t = ts->head;
      res->item.binop.right = nextres;

      break;

    default:
      break;
      // We should never get here...
    }
  }

  return res;
}

ast_t* expr(tokenstream_t* ts, token_t* t) {
  ast_t* res = factor(ts, t);
  ast_t* nextres;

  if(res->type == AST_ERROR)
    return res;

  t = ts->head;

  while(isadditiveoperation(t)) {
    switch(t->type) {
    case PLUS:
      ts->head = t->next;
      free(t);
      t = ts->head;

      nextres = malloc(sizeof(ast_t));
      nextres->type = AST_BINOP;
      nextres->item.binop.type = AST_BINOP_ADD;
      nextres->item.binop.left = res;

      res = nextres;
      nextres = factor(ts, t);
      
      if(nextres->type == AST_ERROR) {
	freeast(res);
	return nextres;
      }
      
      t = ts->head;
      res->item.binop.right = nextres;

      break;
    case MINUS:
      ts->head = t->next;
      free(t);
      t = ts->head;

      nextres = malloc(sizeof(ast_t));
      nextres->type = AST_BINOP;
      nextres->item.binop.type = AST_BINOP_SUB;
      nextres->item.binop.left = res;

      res = nextres;
      nextres = factor(ts, t);
      
      if(nextres->type == AST_ERROR) {
	freeast(res);
	return nextres;
      }
      
      t = ts->head;
      res->item.binop.right = nextres;

      break;

    default:
      // We should never get here...
      break;
    }
  }

  return res;
}

ast_t* statement(tokenstream_t* ts, token_t* t) {
  ast_t* res = expr(ts, t);

  if(res->type == AST_ERROR)
    return res;
  
  t = ts->head;

  if(isend(t)) {
    freetokenstream(ts);
    return res;
  } else {
    freeast(res);

    res = malloc(sizeof(ast_t));
    res->type = AST_ERROR;
    res->item.error = "Syntax error - expected end of statement\n";

    freetokenstream(ts);
    return res;
  }
}

ast_t* parse(tokenstream_t* ts) {
  return statement(ts, ts->head);
}

void freeast(ast_t* t) {
  if(t->type == AST_BINOP) {
    if(t->item.binop.left != NULL) freeast(t->item.binop.left);
    if(t->item.binop.right != NULL) freeast(t->item.binop.right);
  }

  free(t);
}

