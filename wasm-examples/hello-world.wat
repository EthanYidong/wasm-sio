(module
    (type $_sio_stdin_readline (func (param i32)(result i32)))
    (type $_sio_stdout_print (func (param i32 i32)))
    (import "sio" "stdin_readline" (func $stdin_readline (type $_sio_stdin_readline)))
    (import "sio" "stdout_print" (func $stdout_print(type $_sio_stdout_print)))
    (func (export "_start") (local $read_size i32)
        global.get $stack_ptr
        (call $stdin_readline (global.get $stack_ptr))
        local.tee $read_size
        i32.sub
        global.set $stack_ptr
        (call $stdout_print (i32.const 0) (i32.const 7))
        (call $stdout_print (global.get $stack_ptr) (i32.sub (local.get $read_size) (i32.const 1)))
        (call $stdout_print (i32.const 7) (i32.const 2))
    )
    (global $stack_ptr (mut i32) (i32.const 65536))
    (memory $memory (export "memory") 1)
    (data (i32.const 0) "Hello, !\n")
)
