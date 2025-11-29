
use libc::size_t;

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