CLANG=clang-16
LLVM_PROFDATA=llvm-profdata-16
LLVM_COV=llvm-cov-16
NPROC=4
CORPUS_DIR=corpus
FUZZER_BINARY=native_fuzzer
FUZZER_OPTIONS=-max_len=204800 -workers=$(NPROC) -jobs=$(NPROC) $(CORPUS_DIR)
COVERAGE_BINARY=coverage_fuzzer
PROFDATA_FILE=default.profdata
COVERAGE_FILE=c/witness_args_handwritten_reader.h

all: fuzzer coverage

fuzzer:
	$(CLANG) -g -fsanitize=address,fuzzer c_support/fuzzer.c -I c -I c_support -o $(FUZZER_BINARY)

coverage:
	$(CLANG) -fprofile-instr-generate -fcoverage-mapping c_support/fuzzer.c c_support/StandaloneFuzzTargetMain.c -I c -I c_support -o $(COVERAGE_BINARY)

run-fuzzer: fuzzer
	./$(FUZZER_BINARY) $(FUZZER_OPTIONS)

run-coverage: coverage
	./$(COVERAGE_BINARY) $(CORPUS_DIR)/*
	$(LLVM_PROFDATA) merge -sparse *.profraw -o $(PROFDATA_FILE)

$(PROFDATA_FILE): run-coverage

show-coverage: $(PROFDATA_FILE)
	$(LLVM_COV) show coverage_fuzzer -instr-profile=$(PROFDATA_FILE) $(COVERAGE_FILE)

report-coverage: $(PROFDATA_FILE)
	$(LLVM_COV) report coverage_fuzzer -instr-profile=$(PROFDATA_FILE) $(COVERAGE_FILE)

clean:
	rm -rf $(FUZZER_BINARY) $(COVERAGE_BINARY) *.profraw *.profdata *.log

.PHONY: all clean coverage fuzzer run-coverage run-fuzzer show-coverage report-coverage
