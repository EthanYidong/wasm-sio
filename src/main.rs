mod wasm_memory;
mod sio;
mod cli;

use wasmer::*;
use wasmer_wasi::*;

use structopt::StructOpt;

use sio::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    let opts = cli::Opts::from_args();

    let store = Store::default();
    let module = Module::from_file(&store, &opts.wasm_file)?;

    let sio_env = SioEnv::default();
    let sio_imports = import_namespace!({
        "stdin_readline" => Function::new_native_with_env(&store, sio_env.clone(), sio_stdin_readline),
        "stdout_print" => Function::new_native_with_env(&store, sio_env.clone(), sio_stdout_print),
        "stdout_print_num" => Function::new_native_with_env(&store, sio_env.clone(), sio_stdout_print_num)
    });

    let wasi_state = WasiState::new("sio_wasi")
        .arg([1, 2])
        .arg([3, 4])
        .build()?;
    let wasi_env = WasiEnv::new(wasi_state);

    let mut imports = generate_import_object_from_env(&store, wasi_env.clone(), WasiVersion::Latest);
    imports.register("sio", sio_imports);
    
    let instance = Instance::new(&module, &imports)?;
    let run_func: NativeFunc<(), ()> = instance.exports.get_native_function("_start")?;

    run_func.call()?;

    Ok(())
}
