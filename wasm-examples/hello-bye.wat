(module
    (type $_sio_stdin_readline (func (param i32)(result i32)))
    (type $_sio_stdout_print (func (param i32 i32)))
    (import "sio" "stdin_readline" (func $stdin_readline (type $_sio_stdin_readline)))
    (import "sio" "stdout_print" (func $stdout_print(type $_sio_stdout_print)))
    (func $cmp_equal (param $str1_ptr i32) (param $str1_len i32) (param $str2_ptr i32) (param $str2_len i32) (result i32) 
        (local $bytes_compared i32)
        (i32.eq (local.get $str1_len) (local.get $str2_len))
        if (result i32)
            (local.set $bytes_compared (i32.const 0))
            loop (result i32)
                (i32.load8_s (i32.add (local.get $str1_ptr) (local.get $bytes_compared)))
                (i32.load8_s (i32.add (local.get $str2_ptr) (local.get $bytes_compared)))
                i32.eq
                (local.tee $bytes_compared (i32.add (local.get $bytes_compared) (i32.const 1)))
                local.get $str1_len
                i32.eq
                br_if 1
                br 0
            end
        else
            i32.const 0
        end
    )
    (func (export "_start") (local $read_size i32)
        block
            loop
                global.get $stack_ptr
                (call $stdin_readline (global.get $stack_ptr))
                local.tee $read_size
                i32.sub
                global.set $stack_ptr
                (call $cmp_equal (global.get $stack_ptr) (i32.sub (local.get $read_size) (i32.const 1)) (i32.const 9) (i32.const 3))
                br_if 1
                (call $stdout_print (i32.const 0) (i32.const 7))
                (call $stdout_print (global.get $stack_ptr) (i32.sub (local.get $read_size) (i32.const 1)))
                (call $stdout_print (i32.const 7) (i32.const 2))
                br 0
            end
        end
        (call $stdout_print (i32.const 12) (i32.const 5))
    )
    (memory $memory (export "memory") 1)
    (global $stack_ptr (mut i32) (i32.const 65536))
    (data (i32.const 0) "Hello, ") ;; 0..7
    (data (i32.const 7) "!\n") ;; 7..9
    (data (i32.const 9) "bye") ;; 9..12
    (data (i32.const 12) "Bye!\n") ;; 12..17
)
