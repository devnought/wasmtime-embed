use std::ffi::CStr;

use wasmtime::{Caller, Engine, Linker, Module, Store};
use wasmtime_wasi::{ambient_authority, Dir, WasiCtx, WasiCtxBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    let dir = Dir::open_ambient_dir("../", ambient_authority()).unwrap();
    let wasi = WasiCtxBuilder::new()
        .inherit_stdout()
        .preopened_dir(dir, "/")
        .unwrap()
        .build();
    let mut store = Store::new(&engine, wasi);

    let module = Module::from_file(
        &engine,
        "../wasi-guest/target/wasm32-wasi/release/wasi_guest.wasm",
    )?;

    linker
        .get_default(&mut store, "")?
        .typed::<(), (), _>(&store)?
        .call(&mut store, ())?;

    linker.func_wrap(
        "env",
        "host_print",
        |mut caller: Caller<'_, WasiCtx>, ptr_wasm: i32| {
            let memory = caller.get_export("memory").unwrap().into_memory().unwrap();

            let wasm_string = unsafe {
                let ptr_native = memory.data_ptr(&caller).offset(ptr_wasm as isize);
                CStr::from_ptr(ptr_native as *const i8).to_str().unwrap()
            };

            println!("{wasm_string}");
        },
    )?;

    let instance = linker.instantiate(&mut store, &module)?;
    let run = instance.get_typed_func::<(i32, i32), i32, _>(&mut store, "run")?;

    let msg = b"Message from host";
    let offset = 0;
    let mem = instance.get_memory(&mut store, "memory").unwrap();

    mem.write(&mut store, offset, msg)?;
    mem.write(&mut store, offset + msg.len(), &[0u8])?;
    let res = run.call(&mut store, (2, offset as i32))?;

    println!("Returned: {res}");

    Ok(())
}
