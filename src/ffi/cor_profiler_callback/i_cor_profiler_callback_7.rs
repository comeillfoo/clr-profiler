#![allow(non_snake_case)]
use crate::ffi::GUID;

#[repr(C)]
pub struct ICorProfilerCallback7<T> {
    // TODO: fill in FFI interface functions here
}

impl ICorProfilerCallback7<()> {
    // F76A2DBA-1D52-4539-866C-2AA518F9EFC3
    pub const iid: GUID = GUID {
        data1: 0xF76A2DBA,
        data2: 0x1D52,
        data3: 0x4539,
        data4: [0x86, 0x6C, 0x2A, 0xA5, 0x18, 0xF9, 0xEF, 0xC3],
    };
}
