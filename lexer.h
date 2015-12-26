#ifndef __LEXER_H__
#define __LEXER_H__

#include "common.h"

typedef struct token {
  token_e type;
  union {
    int iVal;
    char* errString;
  } item;
} token_t;

typedef struct tokenstream {
  token_t* tok;
  struct tokenstream* next;
} tokenstream_t;

tokenstream_t* lexfullline(char* line);
void freetoken(tokenstream_t* ts);
void freetokenstream(tokenstream_t* s);

#endif
