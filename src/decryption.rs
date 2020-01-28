use std::os::raw::{c_uchar, c_uint};

#[repr(C)]
pub enum EncryptionScheme {
    Unencrypted,
    Cenc,
    Cbcs,
}

#[repr(C)]
pub struct SubsampleEntry {
    pub clear_bytes: c_uint,
    pub cipher_bytes: c_uint,
}

#[repr(C)]
pub struct Pattern {
    pub crypt_byte_block: c_uint,
    pub skip_byte_block: c_uint,
}

#[repr(C)]
pub struct InputBuffer {
    pub data: *const c_uchar,
    pub data_size: c_uint,
    pub encryption_scheme: EncryptionScheme,
    pub key_id: *const c_uchar,
    pub key_id_size: c_uint,
    pub iv: *const c_uchar,
    pub iv_size: c_uint,
    pub subsamples: *const SubsampleEntry,
    pub num_subsamples: c_uint,
    pub pattern: Pattern,
    pub timestamp: u64,
}

#[repr(C)]
pub enum Status {
    Success,
    NeedsMoreData,
    NoKey,
    InitializationError,
    DecryptError,
    DecodeError,
    DeferredInitialization,
}

#[repr(C)]
pub struct DecryptionResult {
    pub status: Status,
    pub data: *mut c_uchar,
    pub capacity: c_uint,
    pub size: c_uint,
}
