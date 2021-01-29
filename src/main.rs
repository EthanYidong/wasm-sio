mod wasm_memory;
mod sio;
mod cli;

use wasmer::*;

use structopt::StructOpt;

use sio::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = cli::Opts::from_args();

    let store = Store::default();
    let module = Module::from_file(&store, &opts.wasm_file)?;

    let sio_env = SioEnv::default();

    let imports = imports!{
        "sio" => {
            "stdin_readline" => Function::new_native_with_env(&store, sio_env.clone(), sio_stdin_readline),
            "stdout_print" => Function::new_native_with_env(&store, sio_env.clone(), sio_stdout_print)
        },
    };

    let instance = Instance::new(&module, &imports)?;
    let run_func: NativeFunc<(), ()> = instance.exports.get_native_function("_start")?;

    run_func.call()?;

    Ok(())
}
