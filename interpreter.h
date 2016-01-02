#ifndef __INTERPRETER_H__
#define __INTERPRETER_H__

#include <parser_helper.h>

typedef enum {
  VAR_UNKNOWN,
  VAR_INT
} vardecl_e;

typedef struct vardecl {
  char* identifier;
  vardecl_e type;

  int mut;
  int initialised;

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
    char* error;
    vardecl_t* decl;
  } item;
} result_t;

typedef struct vmstate {
  vardecl_t* vars;
} vmstate_t;

vardecl_t* getvar(char* name, vmstate_t* state);
vardecl_t* newvar(char* name, vmstate_t* state);

result_t* interpreter_handledecl(ast_decl_t* t, vmstate_t* state);
result_t* interpreter_handlenewident(ast_ident_t* t, vmstate_t* state);
result_t* interpreter_handleident(ast_ident_t* t, vmstate_t* state);
result_t* interpreter_handlebinop(ast_binop_t* t, vmstate_t* state);
result_t* interpreter_handleexpr(ast_expr_t* t, vmstate_t* state);
result_t* interpreter_handleassign(ast_assign_t* t, vmstate_t* state);
result_t* interpreter_handlestmt(ast_stmt_t* t, vmstate_t* state);

void interpreter_handleblock(ast_block_t* block, vmstate_t* state);

void interpretloop(ast_stmt_t* t, vmstate_t* state);

#endif
