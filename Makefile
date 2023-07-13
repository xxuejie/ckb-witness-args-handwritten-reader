CLANG=clang-16
NPROC=4
CORPUS_DIR=corpus
FUZZER_BINARY=native_fuzzer
FUZZER_OPTIONS=-max_len=204800 -workers=$(NPROC) -jobs=$(NPROC) $(CORPUS_DIR)

all: fuzzer

fuzzer:
	$(CLANG) -g -fsanitize=address,fuzzer c/fuzzer.c -I c -o $(FUZZER_BINARY)

run-fuzzer: fuzzer
	./$(FUZZER_BINARY) $(FUZZER_OPTIONS)

.PHONY: all fuzzer run-fuzzer
