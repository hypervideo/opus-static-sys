# About

`opus-static-sys` is an FFI Rust-binding to the reference Opus library `libopus`.

It uses `bindgen` to dynamically generate all Rust bindings and documentation, and will
always compile a **static** version of the underlying Opus library to ensure cross-system
capability without having to ensure that Opus is installed on the target system.

The library is fully `no_std` compatible and represents the minimal amount of overhead
or glue code needed to utilize `libopus` within a Rust project.

## Building

In order to use this crate, you will need both `cmake` and `clang` installed on your
build computer. Most Linux-based operating systems provide these via the built-in package
manager under:

* Clang: `libclang-dev`, `libclang`, `llvm`, or `clang`. On Windows, you may need to manually
install Clang from the [Official Download Page](https://releases.llvm.org/download.html) and
set a corresponding `LIBCLANG_PATH` environment variable.
* Cmake: `cmake` or from the [Clang Download Page](https://cmake.org/download/) for Windows.

## Installation

To use, add the following to your `Cargo.toml` file:

```
[dependencies]
opus-static-sys = "1.0"
```

## License

This library is licensed under the [MIT license](http://opensource.org/licenses/MIT).
