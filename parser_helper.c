#include <stdio.h>
#include <stdlib.h>

#include <parser_helper.h>

ast_stmt_t* statement_assign(ast_assign_t* assign) {
  ast_stmt_t* ret = malloc(sizeof(ast_stmt_t));
  ret->type = AST_STMT_ASSIGN;
  ret->item.assign = assign;

  return ret;
}

ast_stmt_t* statement_expr(ast_expr_t* expr) {
  ast_stmt_t* ret = malloc(sizeof(ast_stmt_t));
  ret->type = AST_STMT_EXPR;
  ret->item.expr = expr;

  return ret;
}

ast_stmt_t* statement_decl(ast_decl_t* decl) {
  ast_stmt_t* ret = malloc(sizeof(ast_stmt_t));
  ret->type = AST_STMT_DECL;
  ret->item.decl = decl;

  return ret;
}

ast_stmt_t* statement_block(ast_block_t* block) {
  ast_stmt_t* ret = malloc(sizeof(ast_stmt_t));
  ret->type = AST_STMT_BLOCK;
  ret->item.block = block;

  return ret;
}

ast_stmt_t* statement_append(ast_stmt_t* statements, ast_stmt_t* new) {
  ast_stmt_t* cur = statements;

  while(cur->next != NULL) {
    cur = cur->next;
  }

  cur->next = new;
  return statements;
}

ast_block_t* block_stmt(ast_stmt_t* first) {
  ast_block_t* ret = malloc(sizeof(ast_block_t));
  ret->first = first;

  return ret;
}

ast_expr_t* expression_int(int val) {
  ast_expr_t* ret = malloc(sizeof(ast_expr_t));
  ret->type = AST_EXPR_INT;
  ret->item.val = val;

  return ret;
}

ast_expr_t* expression_binop(ast_binop_t* binop) {
  ast_expr_t* ret = malloc(sizeof(ast_expr_t));
  ret->type = AST_EXPR_BINOP;
  ret->item.binop = binop;

  return ret;
}

ast_expr_t* expression_ident(ast_ident_t* ident) {
  ast_expr_t* ret = malloc(sizeof(ast_expr_t));
  ret->type = AST_EXPR_IDENT;
  ret->item.ident = ident;

  return ret;
}

ast_binop_t* addition(ast_expr_t* left, ast_expr_t* right) {
  ast_binop_t* ret = malloc(sizeof(ast_binop_t));
  ret->type = AST_BINOP_ADD;
  ret->left = left;
  ret->right = right;

  return ret;
}

ast_binop_t* subtraction(ast_expr_t* left, ast_expr_t* right) {
  ast_binop_t* ret = malloc(sizeof(ast_binop_t));
  ret->type = AST_BINOP_SUB;
  ret->left = left;
  ret->right = right;

  return ret;
}

ast_binop_t* multiplication(ast_expr_t* left, ast_expr_t* right) {
  ast_binop_t* ret = malloc(sizeof(ast_binop_t));
  ret->type = AST_BINOP_MUL;
  ret->left = left;
  ret->right = right;

  return ret;
}

ast_binop_t* division(ast_expr_t* left, ast_expr_t* right) {
  ast_binop_t* ret = malloc(sizeof(ast_binop_t));
  ret->type = AST_BINOP_DIV;
  ret->left = left;
  ret->right = right;

  return ret;
}

ast_binop_t* modulo(ast_expr_t* left, ast_expr_t* right) {
  ast_binop_t* ret = malloc(sizeof(ast_binop_t));
  ret->type = AST_BINOP_MOD;
  ret->left = left;
  ret->right = right;

  return ret;
}

ast_ident_t* identifier(char* ident) {
  ast_ident_t* ret = malloc(sizeof(ast_ident_t));
  ret->ident = ident;

  return ret;
}

ast_decl_t* declaration(ast_ident_t* ident, int mutable) {
  ast_decl_t* ret = malloc(sizeof(ast_decl_t));
  ret->ident = ident;
  ret->mut = mutable;
  ret->type = AST_DECL_TYPE_UNKNOWN;

  return ret;
}

ast_decl_t* declaration_type_i32(ast_ident_t* ident, int mutable) {
  ast_decl_t* ret = malloc(sizeof(ast_decl_t));
  ret->ident = ident;
  ret->mut = mutable;
  ret->type = AST_DECL_TYPE_I32;

  return ret;
}

ast_assign_t* assignment_ident(ast_ident_t* ident, ast_expr_t* value) {
  ast_assign_t* ret = malloc(sizeof(ast_decl_t));
  ret->type = AST_ASSIGN_IDENT;
  ret->item.ident = ident;
  ret->value = value;

  return ret;
}

ast_assign_t* assignment_decl(ast_decl_t* decl, ast_expr_t* value) {
  ast_assign_t* ret = malloc(sizeof(ast_decl_t));
  ret->type = AST_ASSIGN_DECL;
  ret->item.decl = decl;
  ret->value = value;

  return ret;
}
/*
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
*/
