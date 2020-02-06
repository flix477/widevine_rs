use std::os::raw::{c_uchar, c_uint};
use std::slice;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum SessionType {
    Temporary,
    PersistentLicense,
    PermanentKeyRelease,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum InitDataType {
    Cenc,
    KeyIds,
    WebM,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum Exception {
    TypeError,
    NotSupportedError,
    InvalidStateError,
    QuotaExceededError,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum MessageType {
    LicenseRequest,
    LicenseRenewal,
    LicenseRelease,
    IndividualizationRequest,
}

#[derive(Debug, Clone)]
pub struct SessionMessage {
    pub message_type: MessageType,
    pub content: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct SessionEvent {
    pub session_id: String,
    pub data: SessionEventType,
}

#[derive(Debug, Clone)]
pub enum SessionEventType {
    Message(SessionMessage),
    ExpirationChange(f64),
    KeysChange(KeysChange),
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct KeysChange {
    pub has_additional_usable_key: bool,
    pub keys_info: Vec<KeyInformation>,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CDMKeyInformation {
    pub key_id: *const c_uchar,
    pub key_id_size: c_uint,
    pub status: KeyStatus,
    pub system_code: c_uint,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct KeyInformation {
    pub key_id: Vec<u8>,
    pub status: KeyStatus,
    pub system_code: u32,
}

impl Into<KeyInformation> for CDMKeyInformation {
    fn into(self) -> KeyInformation {
        let key_id =
            unsafe { slice::from_raw_parts(self.key_id, self.key_id_size as usize) }.to_vec();

        KeyInformation {
            key_id,
            status: self.status,
            system_code: self.system_code,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum KeyStatus {
    Usable,
    InternalError,
    Expired,
    OutputRestricted,
    OutputDownscaled,
    StatusPending,
    Released,
}
