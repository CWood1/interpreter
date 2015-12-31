#ifndef __PARSER_H__
#define __PARSER_H__

typedef enum {
  VAR_UNKNOWN,
  VAR_INT
} vardecl_e;

typedef struct vardecl {
  char* identifier;
  vardecl_e type;

  int mut;

  union {
    int iVal;
  } item;

  struct vardecl* next;
} vardecl_t;

typedef enum {
  RES_INT,
  RES_ERROR,
  RES_DECL,
  RES_NONE
} result_e;

typedef struct result {
  result_e type;

  union {
    int iVal;
    const char* error;
    vardecl_t* decl;
  } item;
} result_t;

typedef struct vmstate {
  vardecl_t* vars;
} vmstate_t;

typedef enum {
  AST_STMT,
  AST_BINOP,
  AST_INT,
  AST_ERROR,
  AST_DECL,
  AST_IDENT,
  AST_ASSIGN
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
    struct {
      int mut;
      struct ast* ident;
    } decl;
    struct {
      char* ident;
    } ident;
    struct {
      struct ast* ident;
      struct ast* value;
    } assign;
  } item;
} ast_t;

ast_t* statement(ast_t* child);
ast_t* appendstatement(ast_t* statements, ast_t* newstmt);
ast_t* assignment(ast_t* ident, ast_t* expr);
ast_t* declaration(ast_t* ident, int mutable);
ast_t* identifier(char* ident);
ast_t* addition(ast_t* left, ast_t* right);
ast_t* subtraction(ast_t* left, ast_t* right);
ast_t* multiplication(ast_t* left, ast_t* right);
ast_t* division(ast_t* left, ast_t* right);
ast_t* integer(int i);

void printast(ast_t* t);
void freeast(ast_t* t);

#endif
