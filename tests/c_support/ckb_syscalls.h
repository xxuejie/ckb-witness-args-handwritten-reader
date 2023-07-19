#define CKB_SUCCESS 0
#define CKB_INDEX_OUT_OF_BOUND 1
#define CKB_ITEM_MISSING 2
#define CKB_LENGTH_NOT_ENOUGH 3
#define CKB_INVALID_DATA 4

#define SYS_ckb_load_transaction 2051
#define SYS_ckb_load_witness 2074

#include <stddef.h>

size_t __internal_syscall(size_t n, size_t a0, size_t a1, size_t a2, size_t a3,
                          size_t a4, size_t a5);

#define syscall(n, a, b, c, d, e, f)                                           \
  __internal_syscall(n, (long)(a), (long)(b), (long)(c), (long)(d), (long)(e), \
                     (long)(f))
