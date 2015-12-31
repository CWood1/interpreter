#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <ctype.h>

#include <sys/stat.h>

#include "common.h"
#include "lexer.h"
#include "parser.h"

result_t* interpret(ast_t* t, vmstate_t* state);
void interpretloop(ast_t* t, vmstate_t* state);
vardecl_t* getvar(char* name, vmstate_t* state);
vardecl_t* newvar(char* name, vmstate_t* state);

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
    freeast(t);

    return res;
  case AST_ASSIGN:
    {
      if(t->item.assign.ident->type == AST_DECL) {
	result_t* r = interpret(t->item.assign.ident, state);

	if(r->type == RES_DECL) {
	  vd = r->item.decl;
	  free(r);
	} else if(r->type == RES_ERROR) {
	  freeast(t);
	  free(res);
	  return r;
	} else {
	  free(r);
	  freeast(t);

	  res->type = RES_DECL;
	  res->item.error = "Something strange happened\n";
	  return res;
	}
      } else {
	vd = getvar(t->item.assign.ident->item.ident.ident, state);

	if(vd == NULL) {
	  res->type = RES_ERROR;
	  res->item.error = "Error - Reference to undefined variable\n";

	  freeast(t);
	  return res;
	} else if(vd->mut != 1) {
	  res->type = RES_ERROR;
	  res->item.error = "Error - Attempted to assign immutable variable\n";

	  freeast(t);
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
	  freeast(t);
	  return r;
	default:
	  res->type = RES_ERROR;
	  res->item.error = "Error - unexpected return type\n";

	  freeast(t);
	  return res;
	}
      } else {
	switch(vd->type) {
	case VAR_INT:
	  if(r->type != RES_INT) {
	    free(r);
	    res->type = RES_ERROR;
	    res->item.error = "Error - Incompatible types between expression and variable\n";

	    freeast(t);
	    return res;
	  } else {
	    vd->item.iVal = r->item.iVal;
	  }
	default:
	  break;
	}
      }

      res->type = RES_NONE;
      freeast(t);
      return res;
    }
  case AST_INT:
    res->type = RES_INT;
    res->item.iVal = t->item.iVal;

    freeast(t);
    return res;
  case AST_IDENT:
    vd = getvar(t->item.ident.ident, state);

    if(vd == NULL) {
      res->type = RES_ERROR;
      res->item.error = "Error - Reference to undefined variable\n";

      freeast(t);
      return res;
    }

    switch(vd->type) {
    case VAR_UNKNOWN:
      res->type = RES_ERROR;
      res->item.error = "Error - Attempted to reference uninitialised variable\n";

      freeast(t);
      return res;
    case VAR_INT:
      res->type = RES_INT;
      res->item.iVal = vd->item.iVal;

      freeast(t);
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
  ast_t* head = t;
  while(t != NULL) {
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

  freeast(head);
}

int main(int argc, char** argv) {
  if(argc != 2) {
    printf("Expected filename, none found.\n");
    return 1;
  }

  struct stat buf;
  
  if(stat(argv[1], &buf) != 0) {
    printf("Unable to open file %s\n", argv[1]);
    return 1;
  }

  FILE* source = fopen(argv[1], "r");

  if(source == NULL) {
    printf("Unable to open file %s\n", argv[1]);
    return 1;
  }
  
  char* sourcebuf = malloc(buf.st_size);

  if(sourcebuf == NULL) {
    printf("Unable to allocate memory for buffer\n");
    return 1;
  }
  
  fread(sourcebuf, 1, buf.st_size, source);
  fclose(source);

  tokenstream_t* ts = lexfull(sourcebuf, buf.st_size);
  ast_t* ast = parse(ts);
  vmstate_t* state = malloc(sizeof(vmstate_t));
  state->vars = NULL;
  interpretloop(ast, state);
  
  free(sourcebuf);
  return 0;
}
