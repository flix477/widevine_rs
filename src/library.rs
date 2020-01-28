use std::os::raw::c_void;
use std::ptr;

extern "C" {
    fn GetLibraryHandle() -> *mut c_void;
    fn DeinitializeLibrary(library: *mut c_void);
}

pub struct Library(*mut c_void);

impl Library {
    pub fn initialize() -> Result<Self, ()> {
        let library = unsafe { GetLibraryHandle() };
        if library == ptr::null_mut() {
            Err(())
        } else {
            Ok(Self(library))
        }
    }

    pub fn pointer(&self) -> *mut c_void {
        self.0
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        unsafe {
            DeinitializeLibrary(self.0);
        }
    }
}
