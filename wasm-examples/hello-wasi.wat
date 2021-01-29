(module
    (import "sio" "stdin_readline" (func $stdin_readline (param i32) (result i32)))
    (import "sio" "stdout_print" (func $stdout_print (param i32 i32)))
    (import "sio" "stdout_print_num" (func $stdout_print_num (param i32)))

    (import "wasi_snapshot_preview1" "args_sizes_get" (func $_args_sizes_get (param i32 i32) (result i32)))
    (import "wasi_snapshot_preview1" "args_get" (func $_args_get (param i32 i32) (result i32)))
    (func $align (param $align i32) (result i32)
        (local $skipped i32)
        global.get $stack_ptr
        (i32.rem_s (global.get $stack_ptr) (local.get $align))
        local.tee $skipped
        i32.sub
        global.set $stack_ptr
        local.get $skipped
    )
    (func $alloc (param $alloc_bytes i32) (result i32)
        global.get $stack_ptr
        local.get $alloc_bytes
        i32.sub
        global.set $stack_ptr
        global.get $stack_ptr
    )
    (func $dealloc (param $dealloc_bytes i32) (result i32)
        global.get $stack_ptr
        global.get $stack_ptr
        local.get $dealloc_bytes
        i32.add
        global.set $stack_ptr
    )
    (func $args_sizes_get (result i32 i32 i32) 
        (call $alloc (i32.const 4))
        (call $alloc (i32.const 4))
        call $_args_sizes_get
        (i32.load (call $dealloc (i32.const 4)))
        (i32.load (call $dealloc (i32.const 4)))
    )
    (func $args_get (param i32 i32) (result i32 i32 i32 i32 i32)
        (call $alloc (i32.mul (local.get 0) (i32.const 4)))
        (call $alloc (i32.mul (local.get 1) (i32.const 4)))
        call $_args_get
        (i32.load (call $dealloc (i32.const 4)))
        (i32.load (call $dealloc (i32.const 4)))
    )
    (func (export "_start") (local $read_size i32)
        (call $stdin_readline (global.get $stack_ptr))
        local.tee $read_size
        call $alloc
        drop
        (call $align (i32.const 4))
        
        call $args_sizes_get
        call $stdout_print_num
        call $stdout_print_num
        call $stdout_print_num

        (call $stdout_print (i32.const 0) (i32.const 7))
        (call $stdout_print (global.get $stack_ptr) (i32.sub (local.get $read_size) (i32.const 1)))
        (call $stdout_print (i32.const 7) (i32.const 2))
    )
    (memory $memory (export "memory") 2)
    (global $stack_ptr (mut i32) (i32.const 65536))
    (data (i32.const 0) "Hello, ") ;; 0..7
    (data (i32.const 7) "!\n") ;; 7..9
)
