use wasmer::*;

use std::io::{stdin, stdout, prelude::*};

use crate::wasm_memory::*;

#[derive(WasmerEnv, Clone, Default)]
pub struct SioEnv {
    #[wasmer(export)]
    memory: LazyInit<Memory>,
}

pub fn sio_stdin_readline(env: &SioEnv, ptr: i32) -> i32 {
    let memory = env.memory.get_ref().expect("WASM module does not export memory");

    let mut next_line = String::new();
    if let Ok(size) = stdin().read_line(&mut next_line) {
        checked_copy(&memory, ptr, CopyType::ToWasmMemory(next_line.as_bytes()));
        return size as i32;
    }
    0
}

pub fn sio_stdout_print(env: &SioEnv, ptr: i32, len: i32) {
    let memory = env.memory.get_ref().expect("WASM module does not export memory");

    let mut to_write = vec![0; len as usize];
    checked_copy(&memory, ptr, CopyType::FromWasmMemory(&mut to_write));

    let mut stdout = stdout();
    stdout.write_all(&to_write).unwrap();
    stdout.flush().unwrap();
}

pub fn sio_stdout_print_num(env: &SioEnv, num: i32) {
    println!("{}", num);
}
