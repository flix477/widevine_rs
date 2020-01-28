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
    pub session_id: String,
    pub message_type: MessageType,
    pub message: Vec<u8>,
}
