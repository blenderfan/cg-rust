
use libc::c_float;
use libc::c_double;

#[repr(C)]
#[derive(Clone)]
pub struct CVec2f {
    pub x: c_float,
    pub y: c_float
}

#[repr(C)]
#[derive(Clone)]
pub struct CVec2d {
    pub x: c_double,
    pub y: c_double
}
