Pre-requisites:

* I'm running this on a Linux x64 machine
* `rustup +nightly target add armv7-unknown-linux-musleabi`
* `rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu`
* `sudo apt-get install gcc-arm-linux-gnueabi  binutils-arm-linux-gnueabi
  qemu-user` (the first two may not be necessary)

Versions:

* `rustc 1.67.1 (d5a82bbd2 2023-02-07)`
* `clang`:
```
clang version 17.0.0 (https://chromium.googlesource.com/a/external/github.com/llvm/llvm-project 3da83fbafef1689de1fc45c2c3fa3d258edda09d)
Target: x86_64-unknown-linux-gnu
Thread model: posix
```

Steps:

1. `export CLANG_BIN_DIR=` to point to a directory containing `clang`, `lld`
   etc. which are configured to support ARMv7 targets. I used Chromium's,
   because I had it handy (`third_party/llvm-build/Release+Asserts/bin` within
   my Chromium checkout)
2. `CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABI_RUSTFLAGS="-C
   link-arg=-fuse-ld=$CLANG_BIN_DIR/ld.lld -C linker=$CLANG_BIN_DIR/clang" cargo
   +nightly build  --target=armv7-unknown-linux-musleabi -Z build-std`
3. `qemu-arm target/armv7-unknown-linux-musleabi/debug/shifty-business 64 64`
