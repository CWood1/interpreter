#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <ctype.h>

typedef enum {
  INTEGER,
  PLUS,
  MINUS,
  END,
  FIN,
  ERROR
} token_e;

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

typedef struct ast_token {
  token_e type;
  union {
    int iVal;
    char* errString;
    struct {
      struct ast_token* left;
      struct ast_token* right;
    } ptrs;
  } item;
} ast_token_t;

tokenstream_t* lex(char** line) {
  tokenstream_t* currentToken = malloc(sizeof(tokenstream_t));
  currentToken->tok = malloc(sizeof(token_t));
  currentToken->tok->type = END;
  // Until we can fill in more details, after lexing the next token, set the token to EOF
  
  if(**line == '\0')
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
/*
ast_token_t* parse(tokenstream_t ts) {
  ast_token_t* head = malloc(sizeof(ast_token_t));
  ast_token_t* cur = head;
  
  while(ts->tok->type != EOF) {
    switch(ts->tok->type) {
    INTEGER:
      cur->type = INTEGER;
      cur->iVal = ts->tok->iVal;
      break;
    PLUS:
      cur->type = PLUS;
*/    

int main(void) {
  char line[80];
  int done = 0;

  while(done == 0) {
    int first, second;
    
    printf("calc> ");
    fgets(line, 80, stdin);

    tokenstream_t* ts = lexfullline(line);

    switch(ts->tok->type) {
    case INTEGER:
      first = ts->tok->item.iVal;
      ts = ts->next;

      token_e operation = ts->tok->type;
      ts = ts->next;
      
      if(ts->tok->type != INTEGER) {
	printf("Syntax error - unexpected token.\n");
	if(ts->tok->type == ERROR) printf("%s\n", ts->tok->item.errString);
	break;
      }

      second = ts->tok->item.iVal;

      switch(operation) {
      case PLUS:
	printf("%d\n", first + second);
	break;

      case MINUS:
	printf("%d\n", first - second);
	break;

      default:
	printf("Syntax error - unsupported operation\n");
	break;
      }
      
      break;

    case FIN:
      done = 1;
      break;

    default:
      printf("Syntax error - unexpected token\n");
      break;
    }
  }
  
  return 0;
}
