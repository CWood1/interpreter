#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#include "common.h"
#include "lexer.h"

tokenstream_t* lex(char** line) {
  tokenstream_t* currentToken = malloc(sizeof(tokenstream_t));
  currentToken->tok = malloc(sizeof(token_t));
  currentToken->tok->type = END;
  // Until we can fill in more details, after lexing the next token, set the token to EOF
  
  if(**line == '\0' || **line == '\n')
    return currentToken;

  char* lineContents = *line;
  while(isspace(lineContents[0])) lineContents++;

  if(isdigit(lineContents[0])) {
    currentToken->tok->type = INTEGER;
    currentToken->tok->item.iVal = atoi(lineContents);

    while(isdigit((++lineContents)[0]));
    *line = lineContents;

    return currentToken;
  } else if(lineContents[0] == '+') {
    currentToken->tok->type = PLUS;
    *line = ++lineContents;

    return currentToken;
  } else if(lineContents[0] == '-') {
    currentToken->tok->type = MINUS;
    *line = ++lineContents;

    return currentToken;
  } else if(lineContents[0] == '*') {
    currentToken->tok->type = MULTIPLY;
    *line = ++lineContents;

    return currentToken;
  } else if(lineContents[0] == '/') {
    currentToken->tok->type = DIVIDE;
    *line = ++lineContents;

    return currentToken;
  } else if(lineContents[0] == '.') {
    currentToken->tok->type = FIN;
    *line = ++lineContents;

    return currentToken;
  } else {
    currentToken->tok->type = ERROR;
    currentToken->tok->item.errString = "Syntax error.";

    return currentToken;
  }
}

tokenstream_t* lexfullline(char* line) {
  tokenstream_t* head = lex(&line);
  tokenstream_t* cur = head;

  while(cur->tok->type != END && cur->tok->type != ERROR) {
    cur->next = lex(&line);
    cur = cur->next;
  }

  cur->next = NULL;
  return head;
}

void freetoken(tokenstream_t* ts) {
  free(ts->tok);
  free(ts);
}

void freetokenstream(tokenstream_t* s) {
  if(s->next != NULL)
    freetokenstream(s->next);

  freetoken(s);
}
