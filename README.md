Amend `.cargo/config.toml` and `build.rs` to specify your clang/lld locations if needs be.

`cargo build --target=armv7-unknown-linux-musleabi` should give a statically
linked 32-bit ARM target.
`qemu-arm target/armv7-unknown-linux-musleabi/debug/shifty-business 64 64` runs
it.

