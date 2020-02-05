use std::convert::TryInto;
use std::os::raw::{c_uchar, c_uint, c_void};

#[repr(C)]
#[derive(Debug)]
pub struct RemoteBuffer {
    destroy: extern "C" fn(*mut c_void),
    capacity: extern "C" fn(*const c_void) -> c_uint,
    size: extern "C" fn(*const c_void) -> c_uint,
    set_size: extern "C" fn(c_uint, *mut c_void),
    data: extern "C" fn(*mut c_void) -> *mut c_uchar,
}

impl Default for RemoteBuffer {
    fn default() -> Self {
        Self {
            destroy,
            capacity,
            data,
            size,
            set_size,
        }
    }
}

extern "C" fn destroy(target: *mut c_void) {
    unsafe { Box::from_raw(target) };
}

extern "C" fn capacity(target: *const c_void) -> c_uint {
    let target = target as *mut Vec<u8>;
    unsafe { (*target).capacity().try_into().unwrap() }
}

extern "C" fn size(target: *const c_void) -> c_uint {
    let target = target as *mut Vec<u8>;
    unsafe { (*target).len().try_into().unwrap() }
}

extern "C" fn data(target: *mut c_void) -> *mut c_uchar {
    let target = target as *mut Vec<u8>;
    unsafe { (*target).as_mut_ptr() }
}

extern "C" fn set_size(size: c_uint, target: *mut c_void) {
    let target = target as *mut Vec<u8>;
    unsafe {
        (*target).resize(size as usize, 0);
    };
}
