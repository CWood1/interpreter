#ifndef __INTERPRETER_H__
#define __INTERPRETER_H__

#include <parser_helper.h>

result_t* interpret(ast_t* t, vmstate_t* state);
void interpretloop(ast_t* t, vmstate_t* state);
vardecl_t* getvar(char* name, vmstate_t* state);
vardecl_t* newvar(char* name, vmstate_t* state);

#endif
