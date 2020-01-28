use crate::promise_set::{
    FuturePromise, PromiseManager, PromiseResult, PromiseResultData, RejectionInfo,
    INITIALIZED_PROMISE_ID,
};
use crate::remote_buffer::RemoteBuffer;
use crate::types::{Exception, MessageType, SessionMessage};
use std::convert::TryInto;
use std::ffi::CStr;
use std::mem;
use std::os::raw::{c_char, c_uint, c_void};
use std::ptr;
use std::slice;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;

extern "C" {
    fn CreateHost(
        target: *mut Host,
        callback: *mut HostCallback,
        remote_buffer: *mut RemoteBuffer,
    ) -> *mut c_void;
    fn DeinitializeHost(host: *mut c_void);
}

extern "C" fn on_initialized(success: bool, target: *mut Host) {
    println!("Initialized!");
    let result = PromiseResult::Resolved(PromiseResultData::Initialized(success));
    let promise_id = INITIALIZED_PROMISE_ID;

    unsafe { wake_promise(promise_id, result, target) };
}

extern "C" fn on_resolve(promise_id: c_uint, target: *mut Host) {
    println!("Resolved {}", promise_id);
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
    target: *mut Host,
) {
    println!("Rejected {}", promise_id);

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
    target: *mut Host,
) {
    println!("Resolved new session for promise {}", promise_id);
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
    target: *mut Host,
) {
    let session_id = unsafe { CStr::from_ptr(session_id) }.to_string_lossy();
    println!("Got session message for session {}", session_id);
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

// TODO: as_mut_ptr() returns a ptr to the start of the buffer...not the vec
// TODO: buffer pool
extern "C" fn allocate(capacity: c_uint) -> *mut Vec<u8> {
    println!("Allocation");
    let mut buffer = Vec::with_capacity(capacity as usize);
    let pointer = buffer.as_mut_ptr();
    mem::forget(buffer);
    pointer
}

pub unsafe fn wake_promise(promise_id: usize, result: PromiseResult, target: *mut Host) {
    let mut pm = (*target).promise_manager.lock().unwrap();
    pm.finished_promises.insert(promise_id, result);
    pm.wake(promise_id);
}

#[repr(C)]
pub struct HostCallback {
    on_initialized: extern "C" fn(bool, *mut Host),
    on_resolve: extern "C" fn(c_uint, *mut Host),
    on_reject: extern "C" fn(c_uint, Exception, c_uint, *const c_char, c_uint, *mut Host),
    on_resolve_new_session: extern "C" fn(c_uint, *const c_char, c_uint, *mut Host),
    on_session_message:
        extern "C" fn(*const c_char, c_uint, MessageType, *const u8, c_uint, *mut Host),
    allocate: extern "C" fn(c_uint) -> *mut Vec<u8>,
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
        let pointer =
            unsafe { CreateHost(&mut *host, &mut *host.callback, &mut *host.remote_buffer) };

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
