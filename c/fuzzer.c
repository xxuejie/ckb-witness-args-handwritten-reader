#include "binding.c"

typedef struct {
  uint8_t c;
  size_t length;
} data_scratcher_context;

int data_scratcher(const uint8_t *data, size_t length, void *context) {
  data_scratcher_context *c = (data_scratcher_context *)context;
  for (size_t i = 0; i < length; i++) {
    c->c ^= data[i];
  }
  c->length += length;
  return CKB_SUCCESS;
}

int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size) {
  if (size > BUF_SIZE) {
    size = BUF_SIZE;
  }
  set_test_data(data, size, SYS_ckb_load_witness, 111, 222);

  uint8_t buf[32];
  cwhr_cursor_t cursor;
  assert(cwhr_cursor_initialize(&cursor, cwhr_witness_loader_create(111, 222),
                                buf, 32) == CKB_SUCCESS);

  if (cursor.total_length < 1024) {
    uint8_t tmp[1024];
    assert(cwhr_cursor_memcpy(&cursor, tmp) == CKB_SUCCESS);
  }

  cwhr_witness_args_reader_t reader;
  if ((cwhr_witness_args_reader_create(&reader, &cursor) == CKB_SUCCESS) &&
      (cwhr_witness_args_reader_verify(&reader, 0) == CKB_SUCCESS)) {
    if (cwhr_witness_args_reader_has_lock(&reader)) {
      cwhr_bytes_reader_t lock;
      assert(cwhr_witness_args_reader_lock(&reader, &lock) == CKB_SUCCESS);

      data_scratcher_context c;
      c.c = 0;
      c.length = 0;

      assert(cwhr_bytes_reader_read(&lock, data_scratcher, &c) == CKB_SUCCESS);

      assert(cwhr_bytes_reader_length(&lock) == c.length);
    }

    if (cwhr_witness_args_reader_has_input_type(&reader)) {
      cwhr_bytes_reader_t input_type;
      assert(cwhr_witness_args_reader_input_type(&reader, &input_type) ==
             CKB_SUCCESS);

      if (cwhr_bytes_reader_length(&input_type) < 256) {
        uint8_t tmp[256];
        assert(cwhr_bytes_reader_memcpy(&input_type, tmp) == CKB_SUCCESS);
      }
    }

    if (cwhr_witness_args_reader_has_output_type(&reader)) {
      cwhr_bytes_reader_t output_type;
      assert(cwhr_witness_args_reader_output_type(&reader, &output_type) ==
             CKB_SUCCESS);

      data_scratcher_context c;
      c.c = 0;
      c.length = 0;

      assert(cwhr_bytes_reader_read(&output_type, data_scratcher, &c) ==
             CKB_SUCCESS);

      assert(cwhr_bytes_reader_length(&output_type) == c.length);
    }
  }

  return 0;
}
