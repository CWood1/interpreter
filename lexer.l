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
"i32"|"bool"               SAVE_TOKEN; return TTYPE;
"true"                     return TTRUE;
"false"                    return TFALSE;
"=="                       return TEQUALTO;
"!="                       return TNOTEQUAL;
"<"                        return TLESSTHAN;
">"                        return TGREATERTHAN;
"<="                       return TLESSOREQ;
">="                       return TGREATEROREQ;
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
"{"                        return TLBRACE;
"}"                        return TRBRACE;
[a-zA-Z_][a-zA-Z0-9_]*     SAVE_TOKEN; return TIDENTIFIER;
[0-9]+                     SAVE_TOKEN; return TINTEGER;
.                          printf("Error - Unrecognised token %s\n", yytext); yyterminate();

%%
