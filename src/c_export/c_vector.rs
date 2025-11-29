
use libc::c_float;

#[repr(C)]
pub struct CVec2f {
    pub x: c_float,
    pub y: c_float
}

