fn main() {
    println!("cargo:rerun-if-changed=c_support/binding.c");
    println!("cargo:rerun-if-changed=c_support/ckb_syscalls.h");
    println!("cargo:rerun-if-changed=c/witness_args_handwritten_reader.h");

    cc::Build::new()
        .file("c_support/binding.c")
        .include("../c")
        .include("c_support")
        .static_flag(true)
        .flag("-O3")
        .flag("-fno-builtin-printf")
        .flag("-fno-builtin-memcmp")
        .flag("-fdata-sections")
        .flag("-ffunction-sections")
        .flag("-Wall")
        .flag("-Werror")
        .compile("binding");
}
