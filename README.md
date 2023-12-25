liburing bindings with optional LTO

requirements: `apt install make clang lld llvm-dev libclang-dev`

witout LTO: `cargo build`

with LTO: `RUSTFLAGS="-Clinker-plugin-lto -Clinker=clang -Clink-arg=-fuse-ld=lld" cargo build`