use std::os::raw::{c_uchar, c_uint};

#[repr(C)]
#[derive(Debug)]
pub enum EncryptionScheme {
    Unencrypted,
    Cenc,
    Cbcs,
}

#[repr(C)]
#[derive(Debug)]
pub struct SubsampleEntry {
    pub clear_bytes: c_uint,
    pub cipher_bytes: c_uint,
}

#[repr(C)]
#[derive(Debug)]
pub struct Pattern {
    pub crypt_byte_block: c_uint,
    pub skip_byte_block: c_uint,
}

#[repr(C)]
#[derive(Debug)]
pub struct CDMInputBuffer {
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

#[derive(Debug)]
pub struct InputBuffer<'a> {
    pub data: &'a [u8],
    pub encryption_scheme: EncryptionScheme,
    pub key_id: &'a [u8],
    pub iv: &'a [u8],
    pub subsamples: Vec<SubsampleEntry>,
    pub pattern: Pattern,
    pub timestamp: u64,
}

impl Into<CDMInputBuffer> for InputBuffer<'_> {
    fn into(self) -> CDMInputBuffer {
        CDMInputBuffer {
            data: self.data.as_ptr(),
            data_size: self.data.len() as u32,
            encryption_scheme: self.encryption_scheme,
            key_id: self.key_id.as_ptr(),
            key_id_size: self.key_id.len() as u32,
            iv: self.iv.as_ptr(),
            iv_size: self.iv.len() as u32,
            subsamples: std::ptr::null(), // TODO
            num_subsamples: 0,            // TODO
            pattern: self.pattern,
            timestamp: self.timestamp,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
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
#[derive(Debug)]
pub struct DecryptionResult {
    pub status: Status,
    pub data: *mut c_uchar,
    pub capacity: c_uint,
    pub size: c_uint,
}
