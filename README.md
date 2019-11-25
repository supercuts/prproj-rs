# prproj(xml/gz) parser in Rust with wasm

## Questions:

There are two main use cases for Rust and WebAssembly:

- To build an entire application — an entire web app based in Rust. ([yew](https://github.com/yewstack/yew))
- To build a part of an application — using Rust in an existing JavaScript frontend.

&mdash; [Compiling from Rust to WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm) (mozilla.org)

### Should I use Yew?

[Not really ("if you don't want to get your hands dirty").](https://medium.com/swlh/building-a-front-app-with-rust-yes-you-can-and-yew-11e7835d768f)

### Check if file is a gzip

1. If first two bytes == `1f 8b` then YES.
2. Seems like for some reason ungzipped data is `ASCII` and not UTF-8 for some reason.

### Can I use WASM?

[Sometimes.](https://caniuse.com/#search=wasm)

### Does this already exist?
[No.](https://crates.io/search?q=prproj) [No.](https://crates.io/search?q=premiere)
[And no.](https://crates.io/search?q=adobe)

### How to design the interface to easily make a second library just for wasm?

I dunno. 

### How to store floats and/or big values?

- usize
- f32
- f64

- [BigInt](https://crates.io/crates/num)
- [Ramp::Int](https://crates.io/crates/num)

## Debugging Rust in CLion

- https://www.jetbrains.com/help/clion/rust-support.html
- https://blog.jetbrains.com/clion/2019/10/debugging-rust-code-in-clion/
- https://stackoverflow.com/questions/33570021/how-to-set-up-gdb-for-debugging-rust-programs-in-windows

I had to set the Rust toolchain to default to GNU and make the GDB's 64 bit version match
the GNU debugger 64 bit version.

## How to return a Result in JavaScript?

https://stackoverflow.com/questions/55786404/how-to-handle-rusts-errors-from-result-as-a-return-value-instead-of-throwing-an

## Tutorials:

General WASM:
- [Compiling from Rust to WebAssembly (+npm publishing)](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm) (mozilla.org)
- [Shrinking WASM size](https://rustwasm.github.io/book/game-of-life/code-size.html#shrinking-wasm-size)

Web workers in WASM (multi-threading / concurrency):

- [Using WebAssembly with Web Workers ](https://www.sitepen.com/blog/using-webassembly-with-web-workers/) (sitepen.com)
- [Is it possible to run WebAssembly code async?](https://stackoverflow.com/questions/50731650/is-it-possible-to-run-webassembly-code-async) (stackoverflow.com)
- [mbasso/wasm-worker](https://github.com/mbasso/wasm-worker) (github.com)

Opening files in WASM:
- JavaScript interoperability with Rust:
  - [crate `stdweb`](https://docs.rs/stdweb/0.4.20/stdweb/)
    - should I use this though?
- wasm-bindgen:
  - [web_sys::FileReader](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.FileReader.html)
    - This API requires the following crate features to be activated: `Blob`, `FileReader`


## Dependencies:

### wasm:

Parsing XML:

- [minidom](https://crates.io/crates/minidom)

Un-gzip:

- [libflate](https://crates.io/crates/libflate)


### TODO:

Change `/examples` files using less private stuff.
