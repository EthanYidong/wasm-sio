use wasmer::*;

pub enum CopyType<'a> {
    FromWasmMemory(&'a mut [u8]),
    ToWasmMemory(&'a [u8]),
}

impl CopyType<'_> {
    pub fn len(&self) -> usize {
        match self {
            CopyType::FromWasmMemory(v) => v.len(),
            CopyType::ToWasmMemory(v) => v.len(),
        }
    }
}

pub fn checked_copy(memory: &Memory, wasm_ptr: usize, copy: CopyType) {
    let copy_len = copy.len();

    let (copy_start, copy_end) = match copy {
        CopyType::FromWasmMemory(_) =>  (wasm_ptr as i64, wasm_ptr + copy_len),
        CopyType::ToWasmMemory(_) => (wasm_ptr as i64 - copy_len as i64, wasm_ptr),
    };

    if copy_start < 0 || copy_end > memory.data_size() as usize  {
        panic!("Out of bounds: range from {} to {}, max {}", copy_start, wasm_ptr, memory.data_size());
    }

    match copy {
        CopyType::FromWasmMemory(dest) => {
            unsafe { std::ptr::copy_nonoverlapping(memory.data_ptr().add(copy_start as usize), dest.as_mut_ptr(), copy_len) }
        },
        CopyType::ToWasmMemory(src) => {
            unsafe { std::ptr::copy_nonoverlapping(src.as_ptr(), memory.data_ptr().add(copy_start as usize), copy_len) }
        },
    }
}