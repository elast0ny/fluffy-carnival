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
# Release
# The --target argument MUST be provided when building std
# panic_immediate_abort disables any error message printing on panic. This removes all unused strings and filepaths.
cargo +nightly -Zbuild-std-features=panic_immediate_abort build --release --target x86_64-unknown-linux-gnu

# Debug with nice messages and unwind
cargo +nightly build --target x86_64-unknown-linux-gnu
```

## Results

### Windows
Binary size : 32,768 bytes

Strings :

The first string is contains the ones use in main.rs followed by a few strings for errors and module dependencies.
![IDA strings Windows](doc/windows_strings.PNG)

### Linux
Binary size : 38,992 bytes

Strings :
![IDA strings Linux](doc/linux_strings.PNG)