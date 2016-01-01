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

  return vd;
}

result_t* interpret(ast_t* t, vmstate_t* state) {
  result_t* res = malloc(sizeof(result_t));
  vardecl_t* vd;
  
  switch(t->type) {
  case AST_DECL:
    if(t->item.decl.ident->type != AST_IDENT) {
      res->type = RES_ERROR;
      res->item.error = "Syntax error - no identifier found.\n";

      return res;
    }

    vd = getvar(t->item.decl.ident->item.ident.ident, state);

    if(vd != NULL) {
      res->type = RES_ERROR;
      res->item.error = "Error - attempted to declare a variable that already exists\n";

      return res;
    }

    vd = newvar(t->item.decl.ident->item.ident.ident, state);
    
    vd->identifier = t->item.decl.ident->item.ident.ident;
    vd->mut = t->item.decl.mut;
    vd->type = VAR_UNKNOWN;
    vd->next = NULL;
    
    res->type = RES_DECL;
    res->item.decl = vd;

    return res;
  case AST_ASSIGN:
    {
      if(t->item.assign.ident->type == AST_DECL) {
	result_t* r = interpret(t->item.assign.ident, state);

	if(r->type == RES_DECL) {
	  vd = r->item.decl;
	  free(r);
	} else if(r->type == RES_ERROR) {
	  free(res);
	  return r;
	} else {
	  free(r);

	  res->type = RES_DECL;
	  res->item.error = "Something strange happened\n";
	  return res;
	}
      } else {
	vd = getvar(t->item.assign.ident->item.ident.ident, state);

	if(vd == NULL) {
	  res->type = RES_ERROR;
	  char* s = "Errora - Reference to undefined variable ";
	  res->item.error = malloc(strlen(s) + strlen(t->item.assign.ident->item.ident.ident) + 2);
	  sprintf(res->item.error, "%s%s\n", s, t->item.assign.ident->item.ident.ident);

	  return res;
	} else if(vd->mut != 1) {
	  res->type = RES_ERROR;
	  res->item.error = "Error - Attempted to assign immutable variable\n";

	  return res;
	}
      }

      result_t* r = interpret(t->item.assign.value, state);
      if(vd->type == VAR_UNKNOWN) {
	switch(r->type) {
	case RES_INT:
	  vd->type = VAR_INT;
	  vd->item.iVal = r->item.iVal;

	  free(r);
	  break;
	case RES_ERROR:
	  free(res);
	  return r;
	default:
	  res->type = RES_ERROR;
	  res->item.error = "Error - unexpected return type\n";

	  return res;
	}
      } else {
	switch(vd->type) {
	case VAR_INT:
	  if(r->type != RES_INT) {
	    free(r);
	    res->type = RES_ERROR;
	    res->item.error = "Error - Incompatible types between expression and variable\n";

	    return res;
	  } else {
	    vd->item.iVal = r->item.iVal;
	  }
	default:
	  break;
	}
      }

      res->type = RES_NONE;
      return res;
    }
  case AST_INT:
    res->type = RES_INT;
    res->item.iVal = t->item.iVal;

    return res;
  case AST_IDENT:
    vd = getvar(t->item.ident.ident, state);

    if(vd == NULL) {
      res->type = RES_ERROR;
      char* s = "Error - Reference to undefined variable ";
      res->item.error = malloc(strlen(s) + strlen(t->item.ident.ident) + 2);
      sprintf(res->item.error, "%s%s\n", s, t->item.ident.ident);

      return res;
    }

    switch(vd->type) {
    case VAR_UNKNOWN:
      res->type = RES_ERROR;
      res->item.error = "Error - Attempted to reference uninitialised variable\n";

      return res;
    case VAR_INT:
      res->type = RES_INT;
      res->item.iVal = vd->item.iVal;

      return res;
    }
  case AST_BINOP:
    {
      result_t* lt = interpret(t->item.binop.left, state);

      if(lt->type == RES_ERROR) {
	free(res);
	return lt;
      }
      
      result_t* rt = interpret(t->item.binop.right, state);

      if(rt->type == RES_ERROR) {
	free(res);
	free(lt);
	return rt;
      }
      
      int l = lt->item.iVal;
      int r = rt->item.iVal;

      free(lt);
      free(rt);

      t->item.binop.left = NULL;
      t->item.binop.right = NULL;

      res->type = RES_INT;

      switch(t->item.binop.type) {
      case AST_BINOP_ADD:
        res->item.iVal = l + r;
	break;
      case AST_BINOP_SUB:
	res->item.iVal = l - r;
	break;
      case AST_BINOP_MUL:
	res->item.iVal = l * r;
	break;
      case AST_BINOP_DIV:
	res->item.iVal = l / r;
	break;
      case AST_BINOP_MOD:
	res->item.iVal = l % r;
	break;
      }

      return res;
    }
  case AST_ERROR:
    res->type = RES_ERROR;
    res->item.error = t->item.error;

    return res;
  default:
    break;
  }

  res->type = RES_ERROR;
  res->item.error = "Unrecognised AST type.\n";

  return res;
}

void interpretloop(ast_t* t, vmstate_t* state) {  
  if(t->type != AST_STMT) {
    printf("Error - no statement found\n");
    return;
  }
  
  while(t != NULL && t->type == AST_STMT) {
    if(t->type == AST_ERROR) {
      printf("%s\n", t->item.error);
      break;
    }
    
    result_t* res = interpret(t->item.stmt.child, state);

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
    
    t = t->item.stmt.next;
  }
}
