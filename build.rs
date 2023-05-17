fn main() {
    let clang_bin_dir = std::path::PathBuf::from(std::env::var_os("CLANG_BIN_DIR").unwrap());
    let clang = clang_bin_dir.join("clang");
    cc::Build::new()
        .compiler(clang)
        .opt_level_str("z")
        .file("src/shifty.c")
        .compile("shifty");
}
