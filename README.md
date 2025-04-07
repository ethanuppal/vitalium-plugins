# vitalium-plugins

## Prequisites

You will need to install a [Rust](https://rustup.rs) toolchain and make sure `cargo` is in your `$PATH`.
Then, you need to grab [`meson`](https://mesonbuild.com) for building the C++ DSP source code.

You may need to install additional dependencies depending on your system.
See [the submodule's README](./DISTRHO-Ports/README.md) for more information.

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
