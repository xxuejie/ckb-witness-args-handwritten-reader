#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

#include "ckb_syscalls.h"
#define CWHR_DEBUG(...)
// Uncomment the following line for debugging purposes
// #define CWHR_DEBUG(...) fprintf(stderr, __VA_ARGS__)
#include "witness_args_handwritten_reader.h"

#define BUF_SIZE (600 * 1024)

static uint8_t __buf[BUF_SIZE];
static size_t __filled = 0;
static size_t __syscall = 0;
static size_t __index = 0;
static size_t __source = 0;

void set_test_data(void *data, size_t length, size_t syscall, size_t index,
                   size_t source) {
  memcpy(__buf, data, length);
  __filled = length;
  __syscall = syscall;
  __index = index;
  __source = source;
}

size_t __internal_syscall(size_t n, size_t a0, size_t a1, size_t a2, size_t a3,
                          size_t a4, size_t a5) {
  (void)a5;
  if (n != __syscall || __index != a3 || __source != a4) {
    return CKB_INDEX_OUT_OF_BOUND;
  }

  uint8_t *target_addr = (uint8_t *)a0;
  size_t *target_len = (size_t *)a1;
  size_t offset = a2;

  size_t full_size = __filled - offset;
  size_t real_size = *target_len;
  if (real_size > full_size) {
    real_size = full_size;
  }
  memcpy(target_addr, &__buf[offset], real_size);
  *target_len = full_size;
  return 0;
}

cwhr_cursor_t *create_cursor(size_t buf_length, size_t index, size_t source) {
  uint8_t *buf = (uint8_t *)malloc(buf_length);
  cwhr_cursor_t *cursor = (cwhr_cursor_t *)malloc(sizeof(cwhr_cursor_t));

  assert(cwhr_cursor_initialize(cursor,
                                cwhr_witness_loader_create(index, source), buf,
                                buf_length) == CKB_SUCCESS);

  return cursor;
}

void destroy_cursor(cwhr_cursor_t *cursor) {
  free(cursor->buf);
  free(cursor);
}

cwhr_witness_args_reader_t *alloc_witness_args_reader() {
  return (cwhr_witness_args_reader_t *)malloc(sizeof(cwhr_witness_args_reader_t));
}

void free_witness_args_reader(cwhr_witness_args_reader_t *reader) { free(reader); }

cwhr_bytes_reader_t *alloc_bytes_reader() {
  return (cwhr_bytes_reader_t *)malloc(sizeof(cwhr_bytes_reader_t));
}

void free_bytes_reader(cwhr_bytes_reader_t *reader) { free(reader); }
