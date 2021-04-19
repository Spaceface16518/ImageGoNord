# ImageGoNord

This library is a Rust port of [Schrodinger-Hat/ImageGoNord](https://github.com/Schrodinger-Hat/ImageGoNord) designed to be more efficient and portable. It is built on many popular Rust libraries, namely `image`. A majority of the functionality listed in this document is still a work in progress, but this project plans to expose a native Rust library and CLI application, as well as bindings for WASM/JS, Python, and C/C++, with hopes of allowing access from more platforms through the JNI and integrating back into the original ImageGoNord python library using Python bindings.

This library is a perfect fit for fast native applications via multithreading, in-browser conversion via Webassembly, and self-hosted, cloud-native/serverless, or containerized conversion web API's.

*If the license of this project has issues, please open an issue or submit a PR. I am not the best with managing open source licenses, so there may be problems. Thank you for understanding.*

## Usage

### API

ImageGoNord can use several algorithms to convert a given image. These algorithms are controlled by the `Options` type.

The main function in the library is `convert`, which takes in an image, some options, and a palette, and returns a new image that adheres to the color scheme defined by the palette.

### CLI (WIP)

Not yet implemented.

### Environment

#### Rust

Using the library from rust is as simple as adding the crate and using the `convert` function.

```bash
$ cargo add image-go-nord
    Updating 'https://github.com/rust-lang/crates.io-index' index
      Adding image-go-nord v0.1.0 to dependencies
```

or manually add it to your `Cargo.toml`.

```toml
[dependencies]
image-go-nord = "0.1"
```

##### Status

A lot of the Rust port is in progress but there are a lot of key components missing.

#### Webassembly/Javascript (WIP)

Enabling the `wasm-bindgen` feature exposes a WASM-friendly interface to the rust library. This library is compatible with [`wasm-pack`](https://rustwasm.github.io/wasm-pack/), so you can use `wasm-pack` to generate an npm package, complete with Typescript types and JS glue code.

```shell
$ wasm-pack build -- --features wasm-bindgen
```

You can also enable to `wee_alloc` feature to use a smaller, but slower, global allocator. This can help reduce the size of generated wasm files, but it may reduce performance.

You can also enable the `serde` and `serde_json` features to allow serializing conversion options and palettes from JSON. This could be helpful when designing a web API in node.js by removing the need to deserialize user input before passing it to the library, for example.

##### Status

All features are currently in working order, but there is no dedicated interface yet. Look to the `wasm` branch and `src/wasm` directory for progress.

#### C/C++/Cython bindings (WIP)

Enabling the `ffi` feature exports `C`-compatible functions in the crate root. You can build the library as a `cdylib` or `staticlib` to create a shared library for use from other language and environments. While there are ways to pass this option from the command line, it's easier to just put a `crate-type` entry in the `Cargo.toml`.

```toml
[lib]
crate-type = ["cdylib", "staticlib"]
```

When building on Linux, this will produce `libimage_go_nord.so` and `libimage_go_nord.a`. These libraries can then be loaded from other languages.

You can use [`cbindgen`](https://github.com/eqrion/cbindgen) to generate bindings to the Rust library for C, C++, Cython, and other languages that take C bindings, such as Swift and Objective-C. You can place a `cbindgen.toml` configuration file in the crate root or use CLI flags to customize the output.

```shell
$ cbindgen --lang c --crate image-go-nord --output ImageGoNord.h
```

##### Status

Still working out the kinks of generating bindings with conditional compilation.

No work on the ffi module yet.

#### Python (WIP)

Enable the `pyo3` feature to generate a native Python 3 module. You must build the library as a `cdylib` and use [maturin](https://github.com/PyO3/maturin) or [setuptools-rust](https://github.com/PyO3/setuptools-rust) to build the Python module. Read more on the [`pyo3` documentation](https://pyo3.rs/v0.13.2/).

#####

This feature is planned but currently unimplemented.
