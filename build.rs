fn main() {
    println!("cargo:rerun-if-changed=binding.c");
    println!("cargo:rerun-if-changed=c/witness_args_handwritten_reader.h");

    cc::Build::new()
        .file("binding.c")
        .include("c")
        .include("deps/ckb-c-stdlib")
        .include("deps/ckb-c-stdlib/libc")
        .static_flag(true)
        .flag("-O3")
        .flag("-fno-builtin-printf")
        .flag("-fno-builtin-memcmp")
        .flag("-nostdinc")
        .flag("-nostdlib")
        .flag("-fdata-sections")
        .flag("-ffunction-sections")
        .flag("-Wall")
        .flag("-Werror")
        .flag("-Wno-unused-parameter")
        .define("__SHARED_LIBRARY__", None)
        .compile("libwitness_args_handwritten_reader.h");
}
