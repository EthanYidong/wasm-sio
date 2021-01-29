// rustc --crate-type=cdylib --target wasm32-unknown-unknown wasm-examples/hello-rust.rs -o wasm-examples/hello-rust.wasm

#[link(wasm_import_module = "sio")]
extern "C" {
    fn stdin_readline(to: *mut u8) -> i32;
    fn stdout_print(print: *const u8, len: i32);
}

#[no_mangle]
fn _start() {
    let mut name = [0; 1024];
    let hello = "Hello, World!\n";
    unsafe {
        let read = stdin_readline(name.as_mut_ptr());
        stdout_print(hello.as_ptr(), hello.len() as i32);
        stdout_print(name.as_ptr(), read);
    }
}
