# About

`opus-static-sys` is an FFI Rust-binding to the reference Opus library `libopus`.

It uses `bindgen` to dynamically generate all Rust bindings and documentation, and will
always compile a **static** version of the underlying Opus library to ensure cross-system
capability without having to ensure that Opus is installed on the target system.

The library is fully `no_std` compatible and represents the minimal amount of overhead
or glue code needed to utilize `libopus` within a Rust project.

Rust documentation can be found [here](https://docs.rs/opus-static-sys/latest), which
corresponds directly to the `libopus` [documentation](https://opus-codec.org/docs/opus_api-1.5/).

## Building

In order to use this crate, you will need `cmake` installed on your build computer. Most
Linux-based operating systems provide this via the built-in package manager. It is available
on MacOS from `Homebrew`. On Windows, it should have been installed by default when you
installed the Visual Studio compiler.

## Installation

To use, add the following to your `Cargo.toml` file:

```
[dependencies]
opus-static-sys = "1.0"
```

## License

This library is licensed under the [MIT license](http://opensource.org/licenses/MIT).
