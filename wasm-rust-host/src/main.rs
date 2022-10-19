use std::{slice, str};

use wasmtime::{Caller, Engine, Linker, Module, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    let wasi = WasiCtxBuilder::new().inherit_stdout().build();
    let mut store = Store::new(&engine, wasi);

    let module = Module::from_file(
        &engine,
        "../wasm-guest/target/wasm32-wasi/release/wasm_guest.wasm",
    )?;

    linker
        .get_default(&mut store, "")?
        .typed::<(), (), _>(&store)?
        .call(&mut store, ())?;

    linker.func_wrap(
        "env",
        "host_print",
        |mut caller: Caller<'_, WasiCtx>, ptr_wasm: i32, len: i32| {
            let memory = caller.get_export("memory").unwrap().into_memory().unwrap();

            let wasm_string = unsafe {
                let ptr_native = memory.data_ptr(&caller).offset(ptr_wasm as isize);
                let wasm_string_slice = slice::from_raw_parts(ptr_native, len as usize);

                str::from_utf8_unchecked(wasm_string_slice)
            };

            println!("Wasm string      : {wasm_string:?}");
        },
    )?;

    // linker.module(&mut store, "", &module)?;

    let instance = linker.instantiate(&mut store, &module)?;
    let run = instance.get_typed_func::<(i32, i32, i32), i32, _>(&mut store, "run")?;

    let msg = b"Message from host";
    let offset = 0;
    let mem = instance.get_memory(&mut store, "memory").unwrap();

    mem.write(&mut store, offset, msg)?;
    let res = run.call(&mut store, (2, offset as i32, msg.len() as i32))?;

    println!("Wasm returned    : {res}");

    Ok(())
}
