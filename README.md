This is a test project to try to generate the smallest `std` binary.

## Prerequisites
```bash
# Nightly toolchain to allow -Zbuild-std
rustup install nightly
# Rust source to recompile it with the correct profile
rustup +nightly component add rust-src
```

## Build
In order to build, simply use cargo with the nightly toolchain :
```bash
# The --target argument MUST be provided when building std
cargo +nightly run --target [x86_64-pc-windows-msvc|x86_64-unknown-linux-gnu] --release
```

## Results

### Windows
Binary size : 32,768 bytes
Strings :
The first string is contains the ones use in main.rs followed by a few strings for errors and module dependencies.
![IDA strings Windows](doc/windows_strings.PNG)

### Linux
Binary size : TODO
Strings :
TODO