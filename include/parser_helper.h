#ifndef __PARSER_HELPER_H__
#define __PARSER_HELPER_H__

struct ast_expr;
struct ast_block;

typedef struct {
  char* ident;
} ast_ident_t;

typedef struct {
  int mut;
  ast_ident_t* ident;

  char* type;
} ast_decl_t;

typedef struct {
  enum {
    AST_ASSIGN_DECL,
    AST_ASSIGN_IDENT
  } type;

  union {
    ast_decl_t* decl;
    ast_ident_t* ident;
  } item;

  struct ast_expr* value;
} ast_assign_t;

typedef struct {
  enum {
    AST_BINOP_ADD,
    AST_BINOP_SUB,
    AST_BINOP_MUL,
    AST_BINOP_DIV,
    AST_BINOP_MOD
  } type;
  
  struct ast_expr* left;
  struct ast_expr* right;
} ast_binop_t;

typedef struct ast_expr {
  enum {
    AST_EXPR_INT,
    AST_EXPR_BOOL,
    AST_EXPR_BINOP,
    AST_EXPR_IDENT
  } type;

  union {
    int val;
    int bVal;
    ast_binop_t* binop;
    ast_ident_t* ident;
  } item;
} ast_expr_t;

typedef struct ast_stmt {
  enum {
    AST_STMT_ASSIGN,
    AST_STMT_EXPR,
    AST_STMT_DECL,
    AST_STMT_BLOCK
  } type;

  union {
    ast_assign_t* assign;
    ast_expr_t* expr;
    ast_decl_t* decl;
    struct ast_block* block;
  } item;

  struct ast_stmt* next;
} ast_stmt_t;

typedef struct ast_block {
  ast_stmt_t* first;
} ast_block_t;

ast_stmt_t* statement_assign(ast_assign_t* assign);
ast_stmt_t* statement_expr(ast_expr_t* expr);
ast_stmt_t* statement_decl(ast_decl_t* decl);
ast_stmt_t* statement_block(ast_block_t* block);
ast_stmt_t* statement_append(ast_stmt_t* statements, ast_stmt_t* new);

ast_block_t* block_stmt(ast_stmt_t* first);

ast_expr_t* expression_int(int val);
ast_expr_t* expression_bool(int val);
ast_expr_t* expression_binop(ast_binop_t* binop);
ast_expr_t* expression_ident(ast_ident_t* ident);

ast_binop_t* addition(ast_expr_t* left, ast_expr_t* right);
ast_binop_t* subtraction(ast_expr_t* left, ast_expr_t* right);
ast_binop_t* multiplication(ast_expr_t* left, ast_expr_t* right);
ast_binop_t* division(ast_expr_t* left, ast_expr_t* right);
ast_binop_t* modulo(ast_expr_t* left, ast_expr_t* right);

ast_ident_t* identifier(char* ident);

ast_decl_t* declaration(ast_ident_t* ident, int mutable);
ast_decl_t* declaration_type(ast_ident_t* ident, int mutable, char* type);

ast_assign_t* assignment_ident(ast_ident_t* ident, ast_expr_t* value);
ast_assign_t* assignment_decl(ast_decl_t* decl, ast_expr_t* value);

/*
void printast(ast_t* t);
void freeast(ast_t* t);
*/
#endif
