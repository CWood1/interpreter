#ifndef __PARSER_H__
#define __PARSER_H__

typedef enum {
  RES_INT,
  RES_ERROR
} result_e;

typedef struct result {
  result_e type;

  union {
    int iVal;
    const char* error;
  } item;
} result_t;

typedef enum {
  AST_STMT,
  AST_BINOP,
  AST_INT,
  AST_ERROR
} ast_e;

typedef enum {
  AST_BINOP_ADD,
  AST_BINOP_SUB,
  AST_BINOP_MUL,
  AST_BINOP_DIV,
  AST_BINOP_MOD
} ast_binop_e;

typedef struct ast {
  ast_e type;
  union {
    int iVal;
    const char* error;
    struct {
      struct ast* left;
      struct ast* right;
      ast_binop_e type;
    } binop;
    struct {
      struct ast* child;
      struct ast* next;
    } stmt;
  } item;
} ast_t;

int isinteger(token_t* t);
int isend(token_t* t);
int isadditiveoperation(token_t* t);
int ismultiplicativeoperation(token_t* t);
int islparen(token_t* t);
int isrparen(token_t* t);

ast_t* term(tokenstream_t* ts, token_t* t);
ast_t* factor(tokenstream_t* ts, token_t* t);
ast_t* expr(tokenstream_t* ts, token_t* t);
ast_t* statement(tokenstream_t* ts, token_t* t);

ast_t* parse(tokenstream_t* ts);
void freeast(ast_t* a);

#endif
