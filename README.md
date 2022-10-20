# wasmtime-embed
This is a practice project for embedding the `wasmtime` runtime into both
a `dotnet` and `rust` project. Until `wit-bindgen` is more mature, this
is an example of how to pass data that's more complicated than simple
primitive types in and out of the webassembly memory sandbox.

# Running the project
First, compile the `wasm-guest` project. This assumes `cargo-wasi` is
installed.

```
cd wasm-guest
cargo wasi build --release
```

Execute the webassembly library from the host of your choice.

```
cd wasm-dotnet-host
dotnet run

Passed string: Message from host
Hey there!!
Some value: 2
Read 934 characters from README.md
Returned: 4
```

```
cd wasm-rust-host
cargo run

Wasm string      : "Passed string: Message from host"
Wasm string      : "Hey there!!"
Wasm string      : "Some value: 2"
Wasm string      : "Read 934 characters from README.md"
Wasm returned    : 4
```
