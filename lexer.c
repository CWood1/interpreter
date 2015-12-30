#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#include "common.h"
#include "lexer.h"

token_t* lex(char** line, size_t* sz) {
  token_t* tok = malloc(sizeof(token_t));
  tok->type = FIN;
  // Until we can fill in more details, after lexing the next token, set the token to EOF
  
  if(*sz == 0)
    return tok;

  char* lineContents = *line;
  while(*sz != 0 && isspace(lineContents[0])) {
    lineContents++;
    (*sz)--;
  }

  if(*sz == 0)
    return tok;

  if(isdigit(lineContents[0])) {
    tok->type = INTEGER;
    tok->item.iVal = atoi(lineContents);

    (*sz)--;
    lineContents++;

    while(*sz != 0 && isdigit(lineContents[0])) {
      (*sz)--;
      lineContents++;
    }

    *line = lineContents;
    return tok;
  } else if(isalpha(lineContents[0]) || lineContents[0] == '_') {
    if(*sz >= 3 && memcmp(lineContents, "let", 3) == 0 &&
       ((*sz > 3 && isalnum(lineContents[3]) == 0 && lineContents[3] != '_') ||
	(*sz == 3))) {
      tok->type = LET;

      (*sz) -= 3;
      lineContents += 3;

      *line = lineContents;
      return tok;
    } else if(*sz >= 3 && memcmp(lineContents, "mut", 3) == 0 &&
	      ((*sz > 3 && isalnum(lineContents[3]) == 0 && lineContents[3] != '_') ||
	       (*sz == 3))) {
      tok->type = MUT;

      (*sz) -= 3;
      lineContents += 3;

      *line = lineContents;
      return tok;
    } else {
      tok->type = IDENTIFIER;
      char* identstart = lineContents;

      size_t s = *sz;

      while(isalnum(lineContents[0]) || lineContents[0] == '_') {
	lineContents++;
	(*sz)--;
      }

      s -= *sz;

      tok->item.identifier = malloc(s + 1);
      memcpy(tok->item.identifier, identstart, s);
      tok->item.identifier[s] = '\0';

      *line = lineContents;
      return tok;
    }
  } else if(lineContents[0] == '+') {
    tok->type = PLUS;
    *line = ++lineContents;
    (*sz)--;

    return tok;
  } else if(lineContents[0] == '-') {
    tok->type = MINUS;
    *line = ++lineContents;
    (*sz)--;

    return tok;
  } else if(lineContents[0] == '*') {
    tok->type = MULTIPLY;
    *line = ++lineContents;
    (*sz)--;

    return tok;
  } else if(lineContents[0] == '/') {
    tok->type = DIVIDE;
    *line = ++lineContents;
    (*sz)--;

    return tok;
  } else if(lineContents[0] == '%') {
    tok->type = MODULO;
    *line = ++lineContents;
    (*sz)--;

    return tok;
  } else if(lineContents[0] == '(') {
    tok->type = LPAREN;
    *line = ++lineContents;
    (*sz)--;

    return tok;
  } else if(lineContents[0] == ')') {
    tok->type = RPAREN;
    *line = ++lineContents;
    (*sz)--;

    return tok;
  } else if(lineContents[0] == ';') {
    tok->type = END;
    *line = ++lineContents;
    (*sz)--;

    return tok;
  } else if(lineContents[0] == '=') {
    tok->type = EQUALS;
    *line = ++lineContents;
    (*sz)--;

    return tok;
  } else {
    tok->type = ERROR;
    tok->item.errString = "Syntax error.";

    return tok;
  }
}

tokenstream_t* lexfull(char* buf, size_t len) {
  tokenstream_t* ts = malloc(sizeof(tokenstream_t));
  token_t* cur = lex(&buf, &len);

  ts->head = cur;

  while(cur->type != ERROR && cur->type != FIN) {
    cur->next = lex(&buf, &len);
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
