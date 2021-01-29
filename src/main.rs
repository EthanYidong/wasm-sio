use wasmer::*;

use structopt::StructOpt;

use std::io::{stdin, stdout, prelude::*};
use std::path::PathBuf;

#[derive(StructOpt)]
struct Opts {
    #[structopt(parse(from_os_str))]
    wasm_file: PathBuf,
}

#[derive(WasmerEnv, Clone, Default)]
struct SioEnv {
    #[wasmer(export)]
    memory: LazyInit<Memory>,
}

fn sio_stdin_readline(env: &SioEnv, ptr: i32) -> i32 {
    let memory = env.memory.get_ref().expect("WASM module does not export memory");

    let mut next_line = String::new();
    if let Ok(size) = stdin().read_line(&mut next_line) {
        let data = memory.data_ptr();
        if (ptr as usize) < size {
            panic!("Not enough memory: attempted write {} bytes, with only {} bytes left", size, ptr);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(next_line.as_ptr(), data.add(ptr as usize - size), size);
        }
        return size as i32;
    }
    0
}

fn sio_stdout_print(env: &SioEnv, ptr: i32, len: i32) {
    let memory = env.memory.get_ref().expect("WASM module does not export memory");

    let data = memory.data_ptr();
    if ((ptr + len) as u64) > memory.data_size() {
        panic!("Overflow: attempted to access address {}", ptr + len);
    }

    let mut to_write = vec![0; len as usize];
    unsafe {
        std::ptr::copy_nonoverlapping(data.add(ptr as usize), to_write.as_mut_ptr(), len as usize);
    }

    let mut stdout = stdout();
    stdout.write_all(&to_write).unwrap();
    stdout.flush().unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::from_args();

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
