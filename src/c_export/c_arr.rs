
use libc::size_t;
use libc::c_int;
use crate::c_export::c_vector::CVec2f;

#[repr(C)]
pub struct PArray<T> {
    pub data: *const T,
    pub size: size_t
}

impl<T> PArray<T> {

    pub fn new(data: *mut T, size: size_t) -> Self {
        Self {
            data: data,
            size: size
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn cg_rust_free_array_float2(a: *mut PArray<CVec2f>) {
    if !a.is_null() {
        return;
    }
    let _b = unsafe { Box::from_raw(a)};
}

#[unsafe(no_mangle)]
pub extern "C" fn cg_rust_free_array_long(a: *mut PArray<size_t>) {
    if !a.is_null() {
        return;
    }
    let _b = unsafe { Box::from_raw(a)};
}

#[unsafe(no_mangle)]
pub extern "C" fn cg_rust_free_array_int(a: *mut PArray<c_int>) {
    if !a.is_null() {
        return;
    }
    let _b = unsafe { Box::from_raw(a)};
}