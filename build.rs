fn main() {
    println!("cargo:rerun-if-changed=c/binding.c");
    println!("cargo:rerun-if-changed=c/ckb_syscalls.h");
    println!("cargo:rerun-if-changed=c/witness_args_handwritten_reader.h");

    cc::Build::new()
        .file("c/binding.c")
        .include("c")
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
