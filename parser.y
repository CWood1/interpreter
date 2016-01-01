%{
  #include <stdio.h>
  #include <stdlib.h>
  #include <parser_helper.h>
  ast_stmt_t* prog;

  extern int yylex();
  extern int linenum;
  
  void yyerror(const char* s) {
    printf("Error (line %d) - %s\n", linenum, s);
  }
%}

%union {
  ast_stmt_t* stmt;
  ast_expr_t* expr;
  ast_binop_t* binop;
  ast_ident_t* ident;
  ast_decl_t* decl;
  ast_assign_t* assign;
  char* string;
  int token;
}

%debug
%error-verbose

%token <token> TLET TMUT TEQUAL TSTMT TPLUS TMINUS TMULTIPLY TDIVIDE TMOD TLPAREN TRPAREN
%token <token> TCOLON TTYPEI32
%token <string> TIDENTIFIER
%token <string> TINTEGER

%type <stmt> program stmts stmt
%type <assign> var_assign
%type <decl> var_decl var_decl_typed
%type <expr> expr factor term
%type <ident> ident

%%

program : stmts { prog = $1; }

stmts : stmt { $$ = $1; }
| stmts stmt { $$ = statement_append($1, $2); }

stmt : var_assign TSTMT { $$ = statement_assign($1); }
| expr TSTMT { $$ = statement_expr($1); }
| var_decl_typed TSTMT { $$ = statement_decl($1); }

var_assign : var_decl TEQUAL expr { $$ = assignment_decl($1, $3); }
| var_decl_typed TEQUAL expr { $$ = assignment_decl($1, $3); }
| ident TEQUAL expr  { $$ = assignment_ident($1, $3); }

var_decl_typed : TLET ident TCOLON TTYPEI32 { $$ = declaration_type_i32($2, 0); }
| TLET TMUT ident TCOLON TTYPEI32 { $$ = declaration_type_i32($3, 1); }

var_decl : TLET ident { $$ = declaration($2, 0); }
| TLET TMUT ident { $$ = declaration($3, 1); }

ident : TIDENTIFIER { $$ = identifier($1); }

expr : factor { $$ = $1; }
| factor TPLUS expr { $$ = expression_binop(addition($1, $3)); }
| factor TMINUS expr { $$ = expression_binop(subtraction($1, $3)); }

factor : term { $$ = $1; }
| term TMULTIPLY factor { $$ = expression_binop(multiplication($1, $3)); }
| term TDIVIDE factor { $$ = expression_binop(division($1, $3)); }
| term TMOD factor { $$ = expression_binop(modulo($1, $3)); }

term : TINTEGER { $$ = expression_int(atoi($1)); }
| TMINUS TINTEGER { $$ = expression_int(-atoi($2)); }
| ident { $$ = expression_ident($1); }
| TLPAREN expr TRPAREN { $$ = $2; }

%%
