%{
  #include <stdio.h>
  #include <stdlib.h>
  #include <parser_helper.h>
  ast_t* prog;

  extern int yylex();
  void yyerror(const char* s) {
    printf("Error - %s\n", s);
  }
%}

%union {
  ast_t* tree;
  char* string;
  int token;
}

%debug
%error-verbose

%token <token> TLET TMUT TEQUAL TSTMT TPLUS TMINUS TMULTIPLY TDIVIDE TLPAREN TRPAREN
%token <string> TIDENTIFIER
%token <string> TINTEGER

%type <tree> program stmts stmt var_assign var_decl ident expr factor term

%%

program : stmts { prog = $1; }

stmts : stmt { $$ = $1; }
| stmts stmt { $$ = appendstatement($1, $2); }

stmt : var_assign TSTMT { $$ = statement($1); }
| expr TSTMT { $$ = statement($1); }

var_assign : var_decl TEQUAL expr { $$ = assignment($1, $3); }
| ident TEQUAL expr  { $$ = assignment($1, $3); }

var_decl : TLET ident { $$ = declaration($2, 0); }
| TLET TMUT ident { $$ = declaration($3, 1); }

ident : TIDENTIFIER { $$ = identifier($1); }

expr : factor { $$ = $1; }
| factor TPLUS expr { $$ = addition($1, $3); }
| factor TMINUS expr { $$ = subtraction($1, $3); }

factor : term { $$ = $1; }
| term TMULTIPLY factor { $$ = multiplication($1, $3); }
| term TDIVIDE factor { $$ = division($1, $3); }

term : TINTEGER { $$ = integer(atoi($1)); }
| TMINUS TINTEGER { $$ = integer(-atoi($2)); }
| ident { $$ = $1; }
| TLPAREN expr TRPAREN { $$ = $2; }

%%
