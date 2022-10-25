# wasmtime-embed
This is a practice project for embedding the `wasmtime` runtime into both
a `dotnet` and `rust` project. Until `wit-bindgen` is more mature, this
is an example of how to pass data that's more complicated than simple
primitive types in and out of the webassembly memory sandbox.

# Running the project
First, compile the `wasi-guest` project. This assumes `cargo-wasi` is
installed.

```
cd wasi-guest
cargo wasi build --release
```

Execute the webassembly library from the host of your choice.

```
cd wasm-dotnet-host
dotnet run

Wasi was passed string: Message from host
Wasi saysd: Hey there!!
Wasi has some value: 2
Wasi read 1070 characters from LICENSE
Returned: 4
```

```
cd wasm-rust-host
cargo run

Wasm string      : "Wasi was passed string: Message from host"
Wasm string      : "Wasi saysd: Hey there!!"
Wasm string      : "Wasi has some value: 2"
Wasm string      : "Wasi read 1070 characters from LICENSE"
Wasm returned    : 4
```
