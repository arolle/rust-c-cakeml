#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <unistd.h>
#include <assert.h>

extern unsigned char* even_numbers_upto(const unsigned char *a, size_t * alen);
extern void free_char_array(char * ref, size_t len);

void ffieven_numbers_upto (unsigned char *c, long clen, unsigned char *a, long alen) {
  printf("c fficreate_array: available CakeML buffer length %d\n", alen);
  size_t len = alen;
  unsigned char * res = even_numbers_upto(a, &len);
  printf("c fficreate_array: used buffer length %d\n", len);
  assert(len <= alen);
  memcpy(a, res, len);
  free_char_array(res, len);
}

extern unsigned char* even_numbers_upto_malloc_buf(const unsigned char *a, size_t * alen);

void ffieven_numbers_upto_malloc_buf (unsigned char *c, long clen, unsigned char *a, long alen) {
  printf("c ffieven_numbers_upto: available CakeML buffer length %d\n", alen);
  size_t len = alen;
  unsigned char * res = even_numbers_upto_malloc_buf(a, &len);
  printf("c ffieven_numbers_upto: used buffer length %d\n", len);
  assert(len <= alen);
  memcpy(a, res, len);
  free(res);
}

