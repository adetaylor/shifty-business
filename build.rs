fn main() {
    cc::Build::new().compiler("clang").opt_level_str("z").file("src/shifty.c").compile("shifty");
}
