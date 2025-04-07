# vitalium-plugins

## Prequisites

You will need to install a [Rust](https://rustup.rs) toolchain and make sure `cargo` is in your `$PATH`.
Then, you need to grab [`meson`](https://mesonbuild.com) for building the C++ DSP source code.

## Build

First, clone the repository:
```sh
git clone https://github.com/ethanuppal/vitalium-plugins
cd vitalium-plugins
git submodule update --init --recursive
```

Then, run `cargo build`, which will automatically build Vitalium.
```sh
cargo build
```
