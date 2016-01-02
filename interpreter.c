#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#include <parser_helper.h>

#include "interpreter.h"

vardecl_t* getvar(char* name, vmstate_t* state) {
  vardecl_t* vd = state->vars;

  while(vd != NULL &&
	strcmp(vd->identifier, name) != 0) {
    vd = vd->next;
  }

  return vd;
}

vardecl_t* newvar(char* name, vmstate_t* state) {
  vardecl_t* vd;
  
  if(state->vars == NULL) {
    state->vars = malloc(sizeof(vardecl_t));
    vd = state->vars;
  } else {
    vd = state->vars;

    while(vd->next != NULL) {
      vd = vd->next;
    }

    vd->next = malloc(sizeof(vardecl_t));
    vd = vd->next;
  }

  vd->identifier = name;
  vd->initialised = 0;

  return vd;
}

result_t* interpreter_handledecl(ast_decl_t* t, vmstate_t* state) {
  result_t* res = interpreter_handlenewident(t->ident, state);

  if(res->type == RES_ERROR)
    return res;

  res->item.decl->mut = t->mut;

  switch(t->type) {
  case AST_DECL_TYPE_UNKNOWN:
    res->item.decl->type = VAR_UNKNOWN;
    break;

  case AST_DECL_TYPE_I32:
    res->item.decl->type = VAR_INT;
    break;
  }

  return res;
}

result_t* interpreter_handlenewident(ast_ident_t* t, vmstate_t* state) {
  result_t* res = malloc(sizeof(result_t));
  vardecl_t* vd = getvar(t->ident, state);

  if(vd != NULL) {
    res->type = RES_ERROR;

    char* a = "Error - Variable ";
    char* b = " already exists\n";

    res->item.error = malloc(strlen(a) + strlen(b) + strlen(t->ident) + 1);
    sprintf(res->item.error, "%s%s%s", a, t->ident, b);

    return res;
  }

  vd = newvar(t->ident, state);
  vd->type = VAR_UNKNOWN;
  vd->next = NULL;

  res->type = RES_DECL;
  res->item.decl = vd;

  return res;
}

result_t* interpreter_handleident(ast_ident_t* t, vmstate_t* state) {
  result_t* res = malloc(sizeof(result_t));
  vardecl_t* vd;
  
  vd = getvar(t->ident, state);

  if(vd == NULL) {
    res->type = RES_ERROR;
    
    char* s = "Error - Reference to undefined variable ";
    
    res->item.error = malloc(strlen(s) + strlen(t->ident) + 2);
    sprintf(res->item.error, "%s%s\n", s, t->ident);

    return res;
  }

  if(vd->initialised == 0) {
    res->type = RES_ERROR;
    
    char* s = "Error - Reference to uninitialised variable ";
    
    res->item.error = malloc(strlen(s) + strlen(t->ident) + 2);
    sprintf(res->item.error, "%s%s\n", s, t->ident);

    return res;
  }

  switch(vd->type) {
  case VAR_UNKNOWN:
    res->type = RES_ERROR;
    
    char* s = "Error - Attempted to reference uninitialised variable ";
    
    res->item.error = malloc(strlen(s) + strlen(t->ident) + 2);
    sprintf(res->item.error, "%s%s\n", s, t->ident);
    
    return res;
  case VAR_INT:
    res->type = RES_INT;
    res->item.iVal = vd->item.iVal;

    return res;
  }
}

result_t* interpreter_handlebinop(ast_binop_t* t, vmstate_t* state) {
  result_t* res = malloc(sizeof(result_t));
  res->type = RES_INT;

  result_t* left = interpreter_handleexpr(t->left, state);
  result_t* right = interpreter_handleexpr(t->right, state);

  if(left->type == RES_ERROR) {
    free(res);
    free(right);

    return left;
  }

  if(right->type == RES_ERROR) {
    free(res);
    free(left);

    return right;
  }

  switch(t->type) {
  case AST_BINOP_ADD:
    res->item.iVal = left->item.iVal + right->item.iVal;
    break;

  case AST_BINOP_SUB:
    res->item.iVal = left->item.iVal - right->item.iVal;
    break;

  case AST_BINOP_MUL:
    res->item.iVal = left->item.iVal * right->item.iVal;
    break;

  case AST_BINOP_DIV:
    res->item.iVal = left->item.iVal / right->item.iVal;
    break;

  case AST_BINOP_MOD:
    res->item.iVal = left->item.iVal % right->item.iVal;
    break;
  }

  free(left);
  free(right);

  return res;
}

result_t* interpreter_handleexpr(ast_expr_t* t, vmstate_t* state) {
  result_t* res;
  
  switch(t->type) {
  case AST_EXPR_INT:
    res = malloc(sizeof(result_t));
    res->type = RES_INT;
    res->item.iVal = t->item.val;

    return res;

  case AST_EXPR_IDENT:
    return interpreter_handleident(t->item.ident, state);

  case AST_EXPR_BINOP:
    return interpreter_handlebinop(t->item.binop, state);
  }
}

result_t* interpreter_handleassign(ast_assign_t* t, vmstate_t* state) {
  result_t* res;
  vardecl_t* vd;
  
  switch(t->type) {
  case AST_ASSIGN_DECL:
    res = interpreter_handledecl(t->item.decl, state);
       
    if(res->type == RES_ERROR)
      return res;

    vd = res->item.decl;
    free(res);
    
    break;
  case AST_ASSIGN_IDENT:
    vd = getvar(t->item.ident->ident, state);

    if(vd->mut == 0) {
      res = malloc(sizeof(result_t));
      res->type = RES_ERROR;

      char* s = "Error - attempted to assign to immutable variable ";

      res->item.error = malloc(strlen(s) + strlen(vd->identifier) + 2);
      sprintf(res->item.error, "%s%s\n", s, vd->identifier);

      return res;
    }
    
    break;
  }

  res = interpreter_handleexpr(t->value, state);

  if(res->type == RES_ERROR)
    return res;

  switch(res->type) {
  case RES_INT:
    if(vd->type == VAR_INT) {
      vd->item.iVal = res->item.iVal;
      vd->initialised = 1;
    } else if(vd->type == VAR_UNKNOWN && t->type == AST_ASSIGN_DECL) {
      vd->item.iVal = res->item.iVal;
      vd->type = VAR_INT;
      vd->initialised = 1;
    } else {
      free(res);
      res = malloc(sizeof(result_t));

      res->type = RES_ERROR;

      char* s = "Error - type mismatch when assigning to ";
      res->item.error = malloc(strlen(s) + strlen(vd->identifier) + 2);
      sprintf(res->item.error, "%s%s\n", s, vd->identifier);

      return res;
    }
  }

  free(res);
  res = malloc(sizeof(result_t));

  res->type = RES_NONE;
  return res;
}

result_t* interpreter_handlestmt(ast_stmt_t* t, vmstate_t* state) {
  switch(t->type) {
  case AST_STMT_ASSIGN:
    return interpreter_handleassign(t->item.assign, state);
  case AST_STMT_EXPR:
    return interpreter_handleexpr(t->item.expr, state);
  case AST_STMT_DECL:
    return interpreter_handledecl(t->item.decl, state);
  case AST_STMT_BLOCK:
    interpreter_handleblock(t->item.block, state);

    result_t* ret = malloc(sizeof(result_t));
    ret->type = RES_NONE;
    return ret;
  }
}

void interpretloop(ast_stmt_t* t, vmstate_t* state) {
  while(t != NULL) {
    result_t* res = interpreter_handlestmt(t, state);

    switch(res->type) {
    case RES_INT:
      printf("%d\n", res->item.iVal);
      break;
    case RES_ERROR:
      printf("%s\n", res->item.error);
      break;
    default:
      break;
    }

    free(res);
    
    t = t->next;
  }
}

void interpreter_handleblock(ast_block_t* block, vmstate_t* state) {
  interpretloop(block->first, state);
}
