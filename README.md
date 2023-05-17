Steps:

1. `export CLANG_BIN_DIR=` to point to a directory containing `clang`, `lld`
   etc. which are configured to support ARMv7
2. `CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABI_RUSTFLAGS="-C
   link-arg=-fuse-ld=$CLANG_BIN_DIR/ld.lld -C linker=$CLANG_BIN_DIR/clang" cargo
   +nightly build  --target=armv7-unknown-linux-musleabi -Z build-std`
3. `qemu-arm target/armv7-unknown-linux-musleabi/debug/shifty-business 64 64`
