# Example Project hifive revb board setup

## Setup
From http://osblog.stephenmarz.com/ch0.html

1. `rustup override set nightly` in directory
2. `rustup target add riscv32imac-unknown-none-elf`

## Running

This example project is to demonstrate a `std` conflict with `memchr`, a dependency of the `nom` crate.

Run `cargo build` as-is (with nom turned on in Cargo.toml) to show
```
   Compiling memchr v2.4.1
error[E0463]: can't find crate for `std`
  |
  = note: the `riscv32imac-unknown-none-elf` target may not support the standard library
  = note: `std` is required by `memchr` because it does not declare `#![no_std]`
  = help: consider building the standard library from source with `cargo build -Zbuild-std`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.
error: could not compile `memchr`
```

Now comment out:
# [dependencies.nom]
# # version = "7.0.0"
# git = "https://github.com/Geal/nom"
# default-features = false
# features = []

in Cargo.toml

Running `cargo build` now will be successful.