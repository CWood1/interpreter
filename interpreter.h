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

result_t* interpret(ast_t* t, vmstate_t* state);
void interpretloop(ast_t* t, vmstate_t* state);
vardecl_t* getvar(char* name, vmstate_t* state);
vardecl_t* newvar(char* name, vmstate_t* state);

#endif
