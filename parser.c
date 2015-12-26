#include <stdlib.h>

#include "common.h"
#include "lexer.h"
#include "parser.h"

int isfin(tokenstream_t* ts) {
  if(ts->tok->type == FIN)
    return 1;
  else
    return 0;
}

result_t* fin(tokenstream_t* ts) {
  freetokenstream(ts);

  result_t* res = malloc(sizeof(result_t));
  res->type = RES_EXIT;
  
  return res;
}

int isinteger(tokenstream_t* ts) {
  if(ts->tok->type == INTEGER)
    return 1;
  else
    return 0;
}

int isend(tokenstream_t* ts) {
  if(ts->tok->type == END)
    return 1;
  else
    return 0;
}

int isoperation(tokenstream_t* ts) {
  if(ts->tok->type == PLUS)
    return 1;
  else if(ts->tok->type == MINUS)
    return 1;
  else if(ts->tok->type == MULTIPLY)
    return 1;
  else if(ts->tok->type == DIVIDE)
    return 1;
  else
    return 0;
}

int isexpr(tokenstream_t* ts) {
  if(isinteger(ts) == 0)
    return 0;

  ts = ts->next;

  if(isend(ts))
    return 1;

  while(isoperation(ts)) {
    ts = ts->next;

    if(isinteger(ts) == 0)
      return 0;

    ts = ts->next;

    if(isend(ts))
      return 1;
  }

  return 0;
}

result_t* expr(tokenstream_t* ts) {
  result_t* res = malloc(sizeof(result_t));
  tokenstream_t* next;
  
  res->type = RES_INT;
  res->item.iVal = ts->tok->item.iVal;

  next = ts->next;
  freetoken(ts);
  ts = next;

  if(isend(ts)) {
    freetokenstream(ts);
    return res;
  }

  while(isoperation(ts)) {
    switch(ts->tok->type) {
    case PLUS:
      next = ts->next;
      freetoken(ts);
      ts = next;

      res->item.iVal += ts->tok->item.iVal;

      next = ts->next;
      freetoken(ts);
      ts = next;

      break;
    case MINUS:
      next = ts->next;
      freetoken(ts);
      ts = next;

      res->item.iVal -= ts->tok->item.iVal;

      next = ts->next;
      freetoken(ts);
      ts = next;

      break;
    case MULTIPLY:
      next = ts->next;
      freetoken(ts);
      ts = next;

      res->item.iVal *= ts->tok->item.iVal;

      next = ts->next;
      freetoken(ts);
      ts = next;

      break;
    case DIVIDE:
      next = ts->next;
      freetoken(ts);
      ts = next;

      res->item.iVal -= ts->tok->item.iVal;

      next = ts->next;
      freetoken(ts);
      ts = next;

      break;
    }

    if(isend(ts)) {
      freetokenstream(ts);
      return res;
    }
  }
}

result_t* parse(tokenstream_t* ts) {
  if(isfin(ts))
    return fin(ts);
  else if(isexpr(ts))
    return expr(ts);
  else {
    freetokenstream(ts);
    
    result_t* res = malloc(sizeof(result_t));
    res->type = RES_ERROR;
    res->item.error = "Parse error.";
    
    return res;
  }
}
