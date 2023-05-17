Pre-requisites:

* I'm running this on a Linux x64 machine
* `rustup +nightly target add armv7-unknown-linux-musleabi`
* `rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu`
* `sudo apt-get install gcc-arm-linux-gnueabi  binutils-arm-linux-gnueabi
  qemu-user` (the first two may not be necessary)

Steps:

1. Clone this github project
2. `export CLANG_BIN_DIR=` to point to a directory containing `clang`, `lld`
   etc. which are configured to support ARMv7 targets. I used Chromium's,
   because I had it handy (`third_party/llvm-build/Release+Asserts/bin` within
   my Chromium checkout)
3. `CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABI_RUSTFLAGS="-C
   link-arg=-fuse-ld=$CLANG_BIN_DIR/ld.lld -C linker=$CLANG_BIN_DIR/clang" cargo
   +nightly build  --target=armv7-unknown-linux-musleabi -Z build-std`
4. `qemu-arm target/armv7-unknown-linux-musleabi/debug/shifty-business 64 64`

Expected behavior:

* No crash

Actual behavior:

```
thread 'main' panicked at 'attempt to subtract with overflow', /usr/local/google/home/adetaylor/.cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.87/src/int/shift.rs:60:69
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
fatal runtime error: failed to initiate panic, error 3
qemu: uncaught target signal 6 (Aborted) - core dumped
Aborted
```

Versions I happened to use (though I suspect it's fairly version-independent).

* `rustc 1.67.1 (d5a82bbd2 2023-02-07)`
* `clang`:
```
clang version 17.0.0 (https://chromium.googlesource.com/a/external/github.com/llvm/llvm-project 3da83fbafef1689de1fc45c2c3fa3d258edda09d)
Target: x86_64-unknown-linux-gnu
Thread model: posix
```

