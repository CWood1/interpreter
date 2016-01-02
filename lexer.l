%{
#include <string.h>
#include <parser_helper.h>
#include "parser.h"

#define SAVE_TOKEN yylval.string = strdup(yytext)

int linenum;
%}

%option noyywrap

%%

[ \t]                      ;
\n                         linenum++;
"let"                      return TLET;
"mut"                      return TMUT;
"="                        return TEQUAL;
";"                        return TSTMT;
"+"                        return TPLUS;
"-"                        return TMINUS;
"*"                        return TMULTIPLY;
"/"                        return TDIVIDE;
"%"                        return TMOD;
"("                        return TLPAREN;
")"                        return TRPAREN;
":"                        return TCOLON;
"i32"                      return TTYPEI32;
"{"                        return TLBRACE;
"}"                        return TRBRACE;
[a-zA-Z_][a-zA-Z0-9_]*     SAVE_TOKEN; return TIDENTIFIER;
[0-9]+                     SAVE_TOKEN; return TINTEGER;
.                          printf("Error - Unrecognised token %s\n", yytext); yyterminate();

%%
