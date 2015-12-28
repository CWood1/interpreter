#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#include "common.h"
#include "lexer.h"

token_t* lex(char** line) {
  token_t* tok = malloc(sizeof(token_t));
  tok->type = END;
  // Until we can fill in more details, after lexing the next token, set the token to EOF
  
  if(**line == '\0' || **line == '\n')
    return tok;

  char* lineContents = *line;
  while(isspace(lineContents[0])) lineContents++;

  if(isdigit(lineContents[0])) {
    tok->type = INTEGER;
    tok->item.iVal = atoi(lineContents);

    while(isdigit((++lineContents)[0]));
    *line = lineContents;

    return tok;
  } else if(lineContents[0] == '+') {
    tok->type = PLUS;
    *line = ++lineContents;

    return tok;
  } else if(lineContents[0] == '-') {
    tok->type = MINUS;
    *line = ++lineContents;

    return tok;
  } else if(lineContents[0] == '*') {
    tok->type = MULTIPLY;
    *line = ++lineContents;

    return tok;
  } else if(lineContents[0] == '/') {
    tok->type = DIVIDE;
    *line = ++lineContents;

    return tok;
  } else if(lineContents[0] == '(') {
    tok->type = LPAREN;
    *line = ++lineContents;

    return tok;
  } else if(lineContents[0] == ')') {
    tok->type = RPAREN;
    *line = ++lineContents;

    return tok;
  } else if(lineContents[0] == '.') {
    tok->type = FIN;
    *line = ++lineContents;

    return tok;
  } else {
    tok->type = ERROR;
    tok->item.errString = "Syntax error.";

    return tok;
  }
}

tokenstream_t* lexfullline(char* line) {
  tokenstream_t* ts = malloc(sizeof(tokenstream_t));
  token_t* cur = lex(&line);

  ts->head = cur;

  while(cur->type != END && cur->type != ERROR) {
    cur->next = lex(&line);
    cur = cur->next;
  }

  cur->next = NULL;
  return ts;
}

void freetokenstream(tokenstream_t* s) {
  token_t* cur = s->head;
  token_t* next = cur->next;

  while(cur->next != NULL) {
    free(cur);
    cur = next;
    next = cur->next;
  }

  free(cur);
  free(s);
}
