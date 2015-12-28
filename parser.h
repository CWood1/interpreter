#ifndef __PARSER_H__
#define __PARSER_H__

typedef enum {
  RES_INT,
  RES_EXIT,
  RES_ERROR
} result_e;

typedef struct result {
  result_e type;

  union {
    int iVal;
    char* error;
  } item;
} result_t;

typedef enum {
  AST_BINOP,
  AST_INT,
  AST_ERROR
} ast_e;

typedef enum {
  AST_BINOP_ADD,
  AST_BINOP_SUB,
  AST_BINOP_MUL,
  AST_BINOP_DIV
} ast_binop_e;

typedef struct ast {
  ast_e type;
  union {
    int iVal;
    char* error;
    struct {
      struct ast* left;
      struct ast* right;
      ast_binop_e type;
    } binop;
  } item;
} ast_t;

ast_t* parse(tokenstream_t* ts);
void freeast(ast_t* a);

#endif
