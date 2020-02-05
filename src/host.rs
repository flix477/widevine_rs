use crate::promise_set::{
    FuturePromise, PromiseManager, PromiseResult, PromiseResultData, RejectionInfo,
    INITIALIZED_PROMISE_ID,
};
use crate::remote_buffer::RemoteBuffer;
use crate::types::{Exception, MessageType, SessionMessage};
use std::convert::TryInto;
use std::ffi::CStr;
use std::os::raw::{c_char, c_uint, c_void};
use std::ptr;
use std::slice;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;

extern "C" {
    fn CreateHost(
        target: *mut c_void,
        callback: *mut HostCallback,
        remote_buffer: *mut RemoteBuffer,
    ) -> *mut c_void;
    fn DeinitializeHost(host: *mut c_void);
}

extern "C" fn on_initialized(success: bool, target: *mut c_void) {
    let target = target as *mut Host;
    let result = PromiseResult::Resolved(PromiseResultData::Initialized(success));
    let promise_id = INITIALIZED_PROMISE_ID;

    unsafe { wake_promise(promise_id, result, target) };
}

extern "C" fn on_resolve(promise_id: c_uint, target: *mut c_void) {
    let target = target as *mut Host;
    let result = PromiseResult::Resolved(PromiseResultData::None);
    let promise_id: usize = promise_id.try_into().unwrap();

    unsafe { wake_promise(promise_id, result, target) };
}

extern "C" fn on_reject(
    promise_id: c_uint,
    exception: Exception,
    system_code: c_uint,
    error_message: *const c_char,
    _: c_uint,
    target: *mut c_void,
) {
    let target = target as *mut Host;
    let error_message = unsafe { CStr::from_ptr(error_message) };
    let info = RejectionInfo {
        error_message: error_message.to_string_lossy().into_owned(),
        system_code,
        exception,
    };

    let result = PromiseResult::Rejected(info);
    let promise_id: usize = promise_id.try_into().unwrap();
    unsafe { wake_promise(promise_id, result, target) };
}

extern "C" fn on_resolve_new_session(
    promise_id: c_uint,
    session_id: *const c_char,
    _: c_uint,
    target: *mut c_void,
) {
    let target = target as *mut Host;
    let session_id = unsafe { CStr::from_ptr(session_id) }.to_string_lossy();
    let result = PromiseResult::Resolved(PromiseResultData::NewSession(session_id.into_owned()));
    let promise_id: usize = promise_id.try_into().unwrap();

    unsafe { wake_promise(promise_id, result, target) };
}

extern "C" fn on_session_message(
    session_id: *const c_char,
    _: c_uint,
    message_type: MessageType,
    message: *const u8,
    message_length: c_uint,
    target: *mut c_void,
) {
    let target = target as *mut Host;
    let session_id = unsafe { CStr::from_ptr(session_id) }.to_string_lossy();
    let content: &[u8] = unsafe { slice::from_raw_parts(message, message_length as usize) };
    let message = SessionMessage {
        session_id: session_id.to_string(),
        message_type,
        message: content.to_vec(),
    };

    unsafe {
        if let Some(ref sender) = (*target).message_sender {
            sender.send(message).unwrap();
        }
    };
}

// TODO: buffer pool
extern "C" fn allocate(capacity: c_uint) -> *mut c_void {
    let buffer: Box<Vec<u8>> = Box::new(Vec::with_capacity(capacity as usize));
    Box::into_raw(buffer) as *mut c_void
}

pub unsafe fn wake_promise(promise_id: usize, result: PromiseResult, target: *mut Host) {
    let mut pm = (*target).promise_manager.lock().unwrap();
    pm.finished_promises.insert(promise_id, result);
    pm.wake(promise_id);
}

#[repr(C)]
#[derive(Debug)]
pub struct HostCallback {
    on_initialized: extern "C" fn(bool, *mut c_void),
    on_resolve: extern "C" fn(c_uint, *mut c_void),
    on_reject: extern "C" fn(c_uint, Exception, c_uint, *const c_char, c_uint, *mut c_void),
    on_resolve_new_session: extern "C" fn(c_uint, *const c_char, c_uint, *mut c_void),
    on_session_message:
        extern "C" fn(*const c_char, c_uint, MessageType, *const u8, c_uint, *mut c_void),
    allocate: extern "C" fn(c_uint) -> *mut c_void,
}

impl Default for HostCallback {
    fn default() -> Self {
        Self {
            on_initialized,
            on_resolve,
            on_reject,
            on_resolve_new_session,
            on_session_message,
            allocate,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Host {
    pub pointer: *mut c_void,
    initialized: bool,
    callback: Box<HostCallback>,
    promise_manager: Arc<Mutex<PromiseManager>>,
    message_sender: Option<Sender<SessionMessage>>,
    remote_buffer: Box<RemoteBuffer>,
}

impl Default for Host {
    fn default() -> Self {
        Self {
            pointer: ptr::null_mut(),
            initialized: false,
            callback: Box::new(HostCallback::default()),
            promise_manager: Arc::new(Mutex::new(PromiseManager::default())),
            message_sender: None,
            remote_buffer: Box::new(RemoteBuffer::default()),
        }
    }
}

impl Host {
    pub fn initialized(self) -> Result<Box<Self>, ()> {
        let mut host = Box::new(self);
        let pointer = unsafe {
            CreateHost(
                &mut *host as *mut Host as *mut c_void,
                &mut *host.callback,
                &mut *host.remote_buffer,
            )
        };

        if pointer == ptr::null_mut() {
            Err(())
        } else {
            host.pointer = pointer;
            Ok(host)
        }
    }

    pub fn get_future(&mut self, promise_id: usize) -> FuturePromise {
        FuturePromise {
            id: promise_id,
            host: self.promise_manager.clone(),
        }
    }

    pub fn set_message_sender(&mut self, sender: Sender<SessionMessage>) {
        self.message_sender = Some(sender);
    }
}

impl Drop for Host {
    fn drop(&mut self) {
        if self.pointer != ptr::null_mut() {
            unsafe {
                DeinitializeHost(self.pointer);
            }
        }
    }
}

pub struct RemoteHost(*mut c_void);

impl Drop for RemoteHost {
    fn drop(&mut self) {
        if self.0 != ptr::null_mut() {
            unsafe {
                DeinitializeHost(self.0);
            }
        }
    }
}
