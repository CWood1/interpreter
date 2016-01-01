#include <stdio.h>
#include <stdlib.h>

#include <parser_helper.h>

ast_t* statement(ast_t* child) {
  ast_t* ret = malloc(sizeof(ast_t));
  ret->type = AST_STMT;
  ret->item.stmt.child = child;

  return ret;
}

ast_t* appendstatement(ast_t* statements, ast_t* newstmt) {
  ast_t* cur = statements;

  while(cur->item.stmt.next != NULL) {
    cur = cur->item.stmt.next;
  }

  cur->item.stmt.next = newstmt;
  return statements;
}

ast_t* assignment(ast_t* ident, ast_t* expr) {
  ast_t* ret = malloc(sizeof(ast_t));
  ret->type = AST_ASSIGN;
  ret->item.assign.ident = ident;
  ret->item.assign.value = expr;

  return ret;
}

ast_t* declaration(ast_t* ident, int mutable) {
  ast_t* ret = malloc(sizeof(ast_t));
  ret->type = AST_DECL;
  ret->item.decl.mut = mutable;
  ret->item.decl.ident = ident;

  return ret;
}

ast_t* identifier(char* ident) {
  ast_t* ret = malloc(sizeof(ast_t));
  ret->type = AST_IDENT;
  ret->item.ident.ident = ident;

  return ret;
}

ast_t* addition(ast_t* left, ast_t* right) {
  ast_t* ret = malloc(sizeof(ast_t));
  ret->type = AST_BINOP;
  ret->item.binop.type = AST_BINOP_ADD;
  ret->item.binop.left = left;
  ret->item.binop.right = right;

  return ret;
}

ast_t* subtraction(ast_t* left, ast_t* right) {
  ast_t* ret = malloc(sizeof(ast_t));
  ret->type = AST_BINOP;
  ret->item.binop.type = AST_BINOP_SUB;
  ret->item.binop.left = left;
  ret->item.binop.right = right;

  return ret;
}

ast_t* multiplication(ast_t* left, ast_t* right) {
  ast_t* ret = malloc(sizeof(ast_t));
  ret->type = AST_BINOP;
  ret->item.binop.type = AST_BINOP_MUL;
  ret->item.binop.left = left;
  ret->item.binop.right = right;

  return ret;
}

ast_t* division(ast_t* left, ast_t* right) {
  ast_t* ret = malloc(sizeof(ast_t));
  ret->type = AST_BINOP;
  ret->item.binop.type = AST_BINOP_DIV;
  ret->item.binop.left = left;
  ret->item.binop.right = right;

  return ret;
}

ast_t* modulo(ast_t* left, ast_t* right) {
  ast_t* ret = malloc(sizeof(ast_t));
  ret->type = AST_BINOP;
  ret->item.binop.type = AST_BINOP_MOD;
  ret->item.binop.left = left;
  ret->item.binop.right = right;

  return ret;
}

ast_t* integer(int i) {
  ast_t* ret = malloc(sizeof(ast_t));
  ret->type = AST_INT;
  ret->item.iVal = i;

  return ret;
}

void printast(ast_t* t) {
  if(t == NULL) {
    return;
  }
  
  switch(t->type) {
  case AST_INT:
    printf("%d\n", t->item.iVal);
    break;
  case AST_IDENT:
    printf("%s\n",t->item.ident.ident);
    break;
  case AST_DECL:
    printf("Decl ");
    printast(t->item.decl.ident);
    break;
  case AST_BINOP:
    printf("Binop ");
    printast(t->item.binop.left);
    switch(t->item.binop.type) {
    case AST_BINOP_ADD:
      printf("+");
      break;
    case AST_BINOP_SUB:
      printf("-");
      break;
    case AST_BINOP_MUL:
      printf("*");
      break;
    case AST_BINOP_DIV:
      printf("/");
      break;
    }
    printast(t->item.binop.right);
    break;
  case AST_ASSIGN:
    printf("Assign ");
    printast(t->item.assign.ident);
    printast(t->item.assign.value);
    break;
  case AST_STMT:
    printf("Stmt ");
    printast(t->item.stmt.child);
    printast(t->item.stmt.next);
    break;
  }
}

void freeast(ast_t* t) {
  if(t->type == AST_BINOP) {
    if(t->item.binop.left != NULL) freeast(t->item.binop.left);
    if(t->item.binop.right != NULL) freeast(t->item.binop.right);
  } else if(t->type == AST_STMT) {
    if(t->item.stmt.child != NULL) freeast(t->item.stmt.child);
    if(t->item.stmt.next != NULL) freeast(t->item.stmt.next);
  }

  free(t);
}
