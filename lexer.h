#ifndef __LEXER_H__
#define __LEXER_H__

#include "common.h"

typedef enum {
  INTEGER,
  PLUS,
  MINUS,
  MULTIPLY,
  DIVIDE,
  MODULO,
  LPAREN,
  RPAREN,
  IDENTIFIER,
  LET,
  MUT,
  EQUALS,
  END,
  FIN,
  ERROR
} token_e;

typedef struct token {
  token_e type;
  union {
    int iVal;
    const char* errString;
    char* identifier;
  } item;
  struct token* next;
} token_t;

typedef struct tokenstream {
  token_t* head;
} tokenstream_t;

token_t* lex(char** line, size_t* sz);
tokenstream_t* lexfull(char* buf, size_t len);
void freetokenstream(tokenstream_t* s);

#endif
