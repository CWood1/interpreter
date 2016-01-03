#ifndef __INTERPRETER_H__
#define __INTERPRETER_H__

#include <parser_helper.h>

struct scope;

typedef enum {
  VAR_UNKNOWN,
  VAR_NONE,
  VAR_BOOL,
  VAR_INT
} vardecl_e;

typedef struct vardecl {
  char* identifier;
  vardecl_e type;

  int mut;
  int initialised;

  union {
    int iVal;
    int bVal;
  } item;

  struct vardecl* next;
  struct scope* containingScope;
} vardecl_t;

typedef enum {
  RES_INT,
  RES_BOOL,
  RES_ERROR,
  RES_DECL,
  RES_NONE
} result_e;

typedef struct result {
  result_e type;

  union {
    int iVal;
    int bVal;
    char* error;
    vardecl_t* decl;
  } item;
} result_t;

typedef struct scope {
  vardecl_t* vars;
  struct scope* parent;
} scope_t;

vardecl_t* getvar(char* name, scope_t* scope);
vardecl_t* newvar(char* name, scope_t* scope);

result_t* interpreter_handledecl(ast_decl_t* t, scope_t* scope);
result_t* interpreter_handlenewident(ast_ident_t* t, scope_t* scope);
result_t* interpreter_handleident(ast_ident_t* t, scope_t* scope);
result_t* interpreter_handlebinop(ast_binop_t* t, scope_t* scope);
result_t* interpreter_handleexpr(ast_expr_t* t, scope_t* scope);
result_t* interpreter_handleassign(ast_assign_t* t, scope_t* scope);
result_t* interpreter_handlestmt(ast_stmt_t* t, scope_t* scope);
result_t* interpreter_handlecond(ast_cond_t* t, scope_t* scope);

void interpreter_handleblock(ast_block_t* block, scope_t* scope);

void interpretloop(ast_stmt_t* t, scope_t* scope);

#endif
