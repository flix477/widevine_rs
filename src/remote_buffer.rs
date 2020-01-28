use std::convert::TryInto;
use std::os::raw::{c_uchar, c_uint};

#[repr(C)]
pub struct RemoteBuffer {
    destroy: extern "C" fn(*mut Vec<u8>),
    capacity: extern "C" fn(*const Vec<u8>) -> c_uint,
    size: extern "C" fn(*const Vec<u8>) -> c_uint,
    set_size: extern "C" fn(c_uint, *mut Vec<u8>),
    data: extern "C" fn(*mut Vec<u8>) -> *mut c_uchar,
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

// TODO: isn't there an easier way to drop the vec?
extern "C" fn destroy(target: *mut Vec<u8>) {
    println!("Destroying buffer");
    let (length, capacity) = unsafe { ((*target).len(), (*target).capacity()) };
    unsafe { Vec::from_raw_parts(target, length, capacity) };
}

extern "C" fn capacity(target: *const Vec<u8>) -> c_uint {
    println!("Buffer capacity");
    unsafe { (*target).capacity().try_into().unwrap() }
}

extern "C" fn size(target: *const Vec<u8>) -> c_uint {
    println!("Buffer size");
    unsafe { (*target).len().try_into().unwrap() }
}

extern "C" fn data(target: *mut Vec<u8>) -> *mut c_uchar {
    println!("Buffer data");
    unsafe { (*target).as_mut_ptr() }
}

extern "C" fn set_size(size: c_uint, target: *mut Vec<u8>) {
    println!("Buffer set size");
    unsafe {
        (*target).resize(size as usize, 0);
    };
}
