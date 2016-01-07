#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#include <parser_helper.h>

#include "interpreter.h"

vardecl_t* getvar(char* name, scope_t* scope) {
  if(scope == NULL)
    return NULL;
  
  vardecl_t* vd = scope->vars;

  while(vd != NULL && strcmp(vd->identifier, name) != 0) {
    vd = vd->next;
  }

  if(vd == NULL)
    return getvar(name, scope->parent);

  return vd;
}

vardecl_t* newvar(char* name, scope_t* scope) {
  vardecl_t* vd;
  
  if(scope->vars == NULL) {
    scope->vars = malloc(sizeof(vardecl_t));
    vd = scope->vars;
  } else {
    vd = scope->vars;

    while(vd->next != NULL) {
      vd = vd->next;
    }

    vd->next = malloc(sizeof(vardecl_t));
    vd = vd->next;
  }

  vd->identifier = name;
  vd->initialised = 0;
  vd->containingScope = scope;

  vd->next = NULL;

  return vd;
}

result_t* interpreter_handledecl(ast_decl_t* t, scope_t* scope) {
  result_t* res = interpreter_handlenewident(t->ident, scope);

  if(res->type == RES_ERROR)
    return res;

  res->item.decl->mut = t->mut;

  if(t->type == NULL) {
    res->item.decl->type = VAR_UNKNOWN;
  } else if(strcmp(t->type, "i32") == 0) {
    res->item.decl->type = VAR_INT;
  } else if(strcmp(t->type, "bool") == 0) {
    res->item.decl->type = VAR_BOOL;
  }

  return res;
}

result_t* interpreter_handlenewident(ast_ident_t* t, scope_t* scope) {
  result_t* res = malloc(sizeof(result_t));
  vardecl_t* vd = getvar(t->ident, scope);

  if(vd != NULL && vd->containingScope == scope) {
    res->type = RES_ERROR;

    char* a = "Error - Variable ";
    char* b = " already exists\n";

    res->item.error = malloc(strlen(a) + strlen(b) + strlen(t->ident) + 1);
    sprintf(res->item.error, "%s%s%s", a, t->ident, b);

    return res;
  }

  vd = newvar(t->ident, scope);
  vd->type = VAR_UNKNOWN;
  vd->next = NULL;

  res->type = RES_DECL;
  res->item.decl = vd;

  return res;
}

result_t* interpreter_handleident(ast_ident_t* t, scope_t* scope) {
  result_t* res = malloc(sizeof(result_t));
  vardecl_t* vd;
  
  vd = getvar(t->ident, scope);

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
  case VAR_BOOL:
    res->type = RES_BOOL;
    res->item.bVal = vd->item.bVal;

    return res;

  case VAR_NONE:
    res->type = RES_NONE;

    return res;
  }
}

result_t* interpreter_handlebinop(ast_binop_t* t, scope_t* scope) {
  result_t* res = malloc(sizeof(result_t));
  res->type = RES_INT;

  result_t* left = interpreter_handleexpr(t->left, scope);
  result_t* right = interpreter_handleexpr(t->right, scope);

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

  case AST_BINOP_EQUAL:
    res->type = RES_BOOL;

    if(left->type != right->type) {
      res->item.bVal = 0;
    } else if(left->type == RES_INT) {
      res->item.bVal = left->item.iVal == right->item.iVal ? 1 : 0;
    } else if(left->type == RES_BOOL) {
      res->item.bVal = left->item.bVal == right->item.bVal ? 1 : 0;
    }

    break;

  case AST_BINOP_NOTEQUAL:
    res->type = RES_BOOL;

    if(left->type != right->type) {
      res->item.bVal = 1;
    } else if(left->type == RES_INT) {
      res->item.bVal = left->item.iVal == right->item.iVal ? 0 : 1;
    } else if(left->type == RES_BOOL) {
      res->item.bVal = left->item.bVal == right->item.bVal ? 0 : 1;
    }

    break;

  case AST_BINOP_LESSTHAN:
    res->type = RES_BOOL;

    if(left->type != right->type) {
      res->item.bVal = 0;
    } else if(left->type == RES_INT) {
      res->item.bVal = left->item.iVal < right->item.iVal ? 1 : 0;
    } else if(left->type == RES_BOOL) {
      res->item.bVal = 0;
    }

    break;

  case AST_BINOP_GREATERTHAN:
    res->type = RES_BOOL;

    if(left->type != right->type) {
      res->item.bVal = 0;
    } else if(left->type == RES_INT) {
      res->item.bVal = left->item.iVal > right->item.iVal ? 1 : 0;
    } else if(left->type == RES_BOOL) {
      res->item.bVal = 0;
    }

    break;

  case AST_BINOP_LESSOREQ:
    res->type = RES_BOOL;

    if(left->type != right->type) {
      res->item.bVal = 0;
    } else if(left->type == RES_INT) {
      res->item.bVal = left->item.iVal <= right->item.iVal ? 1 : 0;
    } else if(left->type == RES_BOOL) {
      res->item.bVal = 0;
    }

    break;

  case AST_BINOP_GREATEROREQ:
    res->type = RES_BOOL;

    if(left->type != right->type) {
      res->item.bVal = 0;
    } else if(left->type == RES_INT) {
      res->item.bVal = left->item.iVal >= right->item.iVal ? 1 : 0;
    } else if(left->type == RES_BOOL) {
      res->item.bVal = 0;
    }

    break;
  }

  free(left);
  free(right);

  return res;
}

result_t* interpreter_handleexpr(ast_expr_t* t, scope_t* scope) {
  result_t* res;
  
  switch(t->type) {
  case AST_EXPR_INT:
    res = malloc(sizeof(result_t));
    res->type = RES_INT;
    res->item.iVal = t->item.val;

    return res;

  case AST_EXPR_BOOL:
    res = malloc(sizeof(result_t));
    res->type = RES_BOOL;
    res->item.bVal = t->item.bVal;

    return res;
    
  case AST_EXPR_IDENT:
    return interpreter_handleident(t->item.ident, scope);

  case AST_EXPR_BINOP:
    return interpreter_handlebinop(t->item.binop, scope);
    
  case AST_EXPR_BLOCK:
    return interpreter_handleblock(t->item.block, scope);

  case AST_EXPR_IFBLK:
    return interpreter_handlecond(t->item.ifblk, scope);
  }
}

result_t* interpreter_handleassign(ast_assign_t* t, scope_t* scope) {
  result_t* res;
  vardecl_t* vd;
  
  switch(t->type) {
  case AST_ASSIGN_DECL:
    res = interpreter_handledecl(t->item.decl, scope);
       
    if(res->type == RES_ERROR)
      return res;

    vd = res->item.decl;
    free(res);
    
    break;
  case AST_ASSIGN_IDENT:
    vd = getvar(t->item.ident->ident, scope);

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

  res = interpreter_handleexpr(t->value, scope);

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

    break;
    
  case RES_BOOL:
    if(vd->type == VAR_BOOL) {
      vd->item.bVal = res->item.bVal;
      vd->initialised = 1;
    } else if(vd->type == VAR_UNKNOWN && t->type == AST_ASSIGN_DECL) {
      vd->item.bVal = res->item.bVal;
      vd->type = VAR_BOOL;
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

    break;
    
  case RES_NONE:
    if(vd->type == VAR_NONE || (vd->type == VAR_UNKNOWN && t->type == AST_ASSIGN_DECL)) {
      vd->type = VAR_NONE;
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

    break;
  }

  free(res);
  res = malloc(sizeof(result_t));

  res->type = RES_NONE;
  return res;
}

result_t* interpreter_handlecond(ast_cond_t* t, scope_t* scope) {
  result_t* expr = interpreter_handleexpr(t->expr, scope);

  if(expr->type == RES_BOOL && expr->item.bVal == 1) {
    free(expr);
    return interpreter_handleblock(t->block, scope);
  } else if(expr->type != RES_BOOL) {
    free(expr);

    result_t* res = malloc(sizeof(result_t));
    res->type = RES_ERROR;
    res->item.error = strdup("Error - Attempted to branch on non-boolean value\n");

    return res;
  } else {
    free(expr);
    
    switch(t->type) {
    case AST_COND_BARE:
      break;

    case AST_COND_ELSE:
      return interpreter_handleblock(t->item.elseblk, scope);
      break;

    case AST_COND_ELIF:
      return interpreter_handlecond(t->item.elseif, scope);
      break;
    }
  }
  
  result_t* res = malloc(sizeof(result_t));
  res->type = RES_NONE;
  return res;
}

result_t* interpreter_handlestmt(ast_stmt_t* t, scope_t* scope) {
  switch(t->type) {
  case AST_STMT_ASSIGN:
    return interpreter_handleassign(t->item.assign, scope);
  case AST_STMT_EXPR:
    return interpreter_handleexpr(t->item.expr, scope);
  case AST_STMT_DECL:
    return interpreter_handledecl(t->item.decl, scope);
  case AST_STMT_COND:
    return interpreter_handlecond(t->item.cond, scope);
  case AST_STMT_BLOCK:
    return interpreter_handleblock(t->item.block, scope);
  case AST_STMT_WHILE:
    return interpreter_handlewhile(t->item.whileblock, scope);
  case AST_STMT_CONT:
    {
      result_t* res = malloc(sizeof(result_t));
      res->type = RES_CONT;
      
      return res;
    }
  }
}

result_t* interpretloop(ast_stmt_t* t, scope_t* scope) {
  while(t != NULL) {
    result_t* res = interpreter_handlestmt(t, scope);

    switch(res->type) {
    case RES_INT:
      printf("%d\n", res->item.iVal);
      break;
    case RES_BOOL:
      printf("%s\n", res->item.bVal == 1 ? "true" : "false");
      break;
    case RES_ERROR:
      printf("%s\n", res->item.error);
      break;
    case RES_CONT:
      return res;
    default:
      break;
    }

    free(res);
    
    t = t->next;
  }

  result_t* ret = malloc(sizeof(result_t));
  ret->type = RES_NONE;

  return ret;
}

result_t* interpreter_handlewhile(ast_while_t* loop, scope_t* scope) {
  result_t* inloop = interpreter_handleexpr(loop->cond, scope);
  if(inloop->type == RES_ERROR) {
    return inloop;
  } else if(inloop->type != RES_BOOL) {
    free(inloop);

    result_t* res = malloc(sizeof(result_t));
    res->type = RES_ERROR;
    res->item.error = strdup("Error - Attempted to execute while loop with non-boolean condition\n");

    return res;
  }

  while(inloop->item.bVal == 1) {
    result_t* tmp = interpreter_handleblock(loop->block, scope);

    if(tmp->type == RES_ERROR) {
      free(inloop);
      return tmp;
    }

    free(inloop);
    
    inloop = interpreter_handleexpr(loop->cond, scope);
    if(inloop->type == RES_ERROR) {
      return inloop;
    } else if(inloop->type != RES_BOOL) {
      free(inloop);

      result_t* res = malloc(sizeof(result_t));
      res->type = RES_ERROR;
      res->item.error = strdup("Error - Attempted to execute while loop with non-boolean condition\n");

      return res;
    }
  }

  free(inloop);

  inloop = malloc(sizeof(result_t));
  inloop->type = RES_NONE;
  return inloop;
}

result_t* interpreter_handleblock(ast_block_t* block, scope_t* scope) {
  scope_t* newScope = malloc(sizeof(scope_t));
  newScope->parent = scope;
  newScope->vars = NULL;

  if(block->first != NULL) {
    result_t* res = interpretloop(block->first, newScope);

    if(res->type = RES_CONT)
      return res;
  }

  if(block->last != NULL) {
    return interpreter_handleexpr(block->last, scope);
  } else {
    result_t* res = malloc(sizeof(result_t));
    res->type = RES_NONE;

    return res;
  }
}
