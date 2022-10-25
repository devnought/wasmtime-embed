using System.Text;
using Wasmtime;

using var engine = new Engine();
using var module = Module.FromFile(engine, "../wasi-guest/target/wasm32-wasi/release/wasi_guest.wasm");
using var linker = new Linker(engine);
using var store = new Store(engine);

var wasiConfig = new WasiConfiguration()
    .WithInheritedStandardOutput()
    .WithPreopenedDirectory("../", "/");

store.SetWasiConfiguration(wasiConfig);
linker.DefineWasi();

linker.Define(
    "env",
    "host_print",
    Function.FromCallback(store, (Caller caller, int address) =>
    {
        var message = caller.GetMemory("memory")?.ReadNullTerminatedString(address) ?? "NO DATA";
        Console.WriteLine(message);
    })
);

var instance = linker.Instantiate(store, module);
var memory = instance.GetMemory("memory");
var offset = 0;
var message = "Message from host";
memory?.WriteString(offset, message, Encoding.UTF8);
var run = instance.GetFunction<int, int, int>("run");

if (run is null)
{
    Console.WriteLine("error: run export is missing");
    return;
}

var test = run(2, offset);
Console.WriteLine($"Returned: {test}");
