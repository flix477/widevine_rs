use crate::decryption::{CDMInputBuffer, InputBuffer};
use crate::decryption::{DecryptionResult, Status};
use crate::host::Host;
use crate::timer::Timer;
use crate::types::{InitDataType, SessionType};
use crate::Library;
use std::convert::TryInto;
use std::os::raw::{c_uchar, c_uint, c_void};
use std::ptr;

extern "C" {
    fn GetCDM(library: *mut c_void, host: *mut c_void) -> *mut c_void;
    fn CDM_Initialize(cdm: *mut c_void);
    fn CDM_SetServerCertificate(
        cdm: *mut c_void,
        promise_id: c_uint,
        certificate: *const c_uchar,
        certificate_length: c_uint,
    );
    fn CDM_CreateSessionAndGenerateRequest(
        cdm: *mut c_void,
        promise_id: c_uint,
        session_type: SessionType,
        init_data_type: InitDataType,
        init_data: *const c_uchar,
        init_data_size: c_uint,
    );
    fn CDM_UpdateSession(
        cdm: *mut c_void,
        promise_id: c_uint,
        session_id: *const c_uchar,
        session_id_size: c_uint,
        response: *const c_uchar,
        response_size: c_uint,
    );
    fn CDM_Decrypt(cdm: *mut c_void, encrypted_buffer: CDMInputBuffer) -> DecryptionResult;
    fn CDM_TimerExpired(cdm: *mut c_void, context: *mut c_void);
    fn DeinitializeCDM(cdm: *mut c_void);
}

pub struct CDM(*mut c_void);

impl CDM {
    pub fn initialize(library: &Library, host: &Host) -> Result<Self, ()> {
        let cdm = unsafe { GetCDM(library.pointer(), host.pointer) };
        if cdm == ptr::null_mut() {
            Err(())
        } else {
            Ok(Self(cdm))
        }
    }

    pub fn request_initialization(&mut self) {
        unsafe { CDM_Initialize(self.0) };
    }

    pub fn set_server_certificate(&mut self, promise_id: usize, certificate: &[u8]) {
        unsafe {
            CDM_SetServerCertificate(
                self.0,
                promise_id.try_into().unwrap(),
                certificate.as_ptr(),
                certificate.len().try_into().unwrap(),
            )
        }
    }

    pub fn create_session(
        &mut self,
        promise_id: usize,
        session_type: SessionType,
        init_data_type: InitDataType,
        init_data: &[u8],
    ) {
        unsafe {
            CDM_CreateSessionAndGenerateRequest(
                self.0,
                promise_id.try_into().unwrap(),
                session_type,
                init_data_type,
                init_data.as_ptr(),
                init_data.len().try_into().unwrap(),
            );
        }
    }

    pub fn update_session(&mut self, promise_id: usize, session_id: &str, response: &[u8]) {
        unsafe {
            CDM_UpdateSession(
                self.0,
                promise_id.try_into().unwrap(),
                session_id.as_ptr(),
                session_id.len().try_into().unwrap(),
                response.as_ptr(),
                response.len().try_into().unwrap(),
            );
        }
    }

    // TODO: not nicely typed because Status::Success exists
    pub fn decrypt(&mut self, input: InputBuffer) -> Result<Vec<u8>, Status> {
        let result = unsafe { CDM_Decrypt(self.0, input.into()) };
        if let Status::Success = result.status {
            let data = unsafe {
                Vec::from_raw_parts(result.data, result.size as usize, result.capacity as usize)
            };
            Ok(data)
        } else {
            Err(result.status)
        }
    }

    pub fn timer_expired(&mut self, timer: Timer) {
        unsafe { CDM_TimerExpired(self.0, timer.context) }
    }
}

impl Drop for CDM {
    fn drop(&mut self) {
        unsafe {
            DeinitializeCDM(self.0);
        }
    }
}
