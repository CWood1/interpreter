#ifndef __PARSER_H__
#define __PARSER_H__

typedef enum {
  RES_INT,
  RES_EXIT,
  RES_ERROR
} result_e;

typedef struct result {
  result_e type;

  union {
    int iVal;
    char* error;
  } item;
} result_t;

result_t* parse(tokenstream_t* ts);

#endif
