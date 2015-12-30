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

int isident(token_t* t);
int islet(token_t* t);
int ismut(token_t* t);
int isequals(token_t* t);
int isinteger(token_t* t);
int isend(token_t* t);
int isadditiveoperation(token_t* t);
int ismultiplicativeoperation(token_t* t);
int islparen(token_t* t);
int isrparen(token_t* t);

ast_t* term(tokenstream_t* ts, token_t* t);
ast_t* factor(tokenstream_t* ts, token_t* t);
ast_t* expr(tokenstream_t* ts, token_t* t);

ast_t* identifier(tokenstream_t* ts, token_t* t);
ast_t* declare(tokenstream_t* ts, token_t* t);
ast_t* assignment(tokenstream_t* ts, token_t* t);

ast_t* statement(tokenstream_t* ts, token_t* t);

ast_t* parse(tokenstream_t* ts);
void freeast(ast_t* a);

#endif
