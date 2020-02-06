mod cdm;
pub mod decryption;
mod host;
mod library;
mod promise_set;
mod remote_buffer;
mod timer;
pub mod types;

use cdm::CDM;
use decryption::{InputBuffer, Status};
use host::Host;
use library::Library;
use promise_set::{PromiseResultData, PromiseSet, RejectionInfo, INITIALIZED_PROMISE_ID};
use std::sync::mpsc::Sender;
use types::{InitDataType, SessionEvent, SessionType};

#[derive(Clone, Debug)]
pub enum InitializeCDMError {
    Failed,
    Rejected(RejectionInfo),
}

#[derive(Clone, Debug)]
pub enum CreateSessionError {
    Failed, // TODO: this is a result of badly typing the promise system
    Rejected(RejectionInfo),
}

pub struct WidevineAPI {
    cdm: CDM,
    host: Box<Host>,
    #[allow(dead_code)]
    library: Library,
    promise_set: PromiseSet,
}

impl WidevineAPI {
    pub fn initialize() -> Result<Self, ()> {
        let library = Library::initialize()?;
        let host = Host::default().initialized()?;
        let cdm = CDM::initialize(&library, &host)?;
        let promise_set = PromiseSet::default();

        Ok(Self {
            library,
            host,
            cdm,
            promise_set,
        })
    }

    pub async fn initialize_cdm(&mut self) -> Result<(), InitializeCDMError> {
        self.cdm.request_initialization();
        let result = self
            .host
            .get_future(INITIALIZED_PROMISE_ID)
            .await
            .as_result();
        match result {
            Ok(PromiseResultData::Initialized(true)) => Ok(()),
            Err(info) => Err(InitializeCDMError::Rejected(info)),
            _ => Err(InitializeCDMError::Failed),
        }
    }

    pub async fn set_server_certificate(
        &mut self,
        certificate: &[u8],
    ) -> Result<(), RejectionInfo> {
        let promise_id = self.promise_set.create();
        self.cdm.set_server_certificate(promise_id, certificate);
        self.host.get_future(promise_id).await.as_result()?;
        self.promise_set.pop(promise_id);
        Ok(())
    }

    pub async fn create_session(
        &mut self,
        session_type: SessionType,
        init_data_type: InitDataType,
        init_data: Vec<u8>, // TODO: using slice instead gives E0700
        sender: Sender<SessionEvent>,
    ) -> Result<String, CreateSessionError> {
        let promise_id = self.promise_set.create();
        self.host.set_event_sender(sender);
        self.cdm
            .create_session(promise_id, session_type, init_data_type, &init_data);
        let result = self.host.get_future(promise_id).await.as_result();
        self.promise_set.pop(promise_id);

        match result {
            Ok(PromiseResultData::NewSession(id)) => Ok(id),
            Err(info) => Err(CreateSessionError::Rejected(info)),
            _ => Err(CreateSessionError::Failed),
        }
    }

    // TODO: refactor promises, lots of repeated code here
    pub async fn update_session(
        &mut self,
        session_id: &str,
        response: &[u8],
    ) -> Result<(), RejectionInfo> {
        let promise_id = self.promise_set.create();
        self.cdm.update_session(promise_id, session_id, response);
        let _result = self.host.get_future(promise_id).await.as_result()?;
        self.promise_set.pop(promise_id);
        Ok(())
    }

    pub fn decrypt(&mut self, input_buffer: InputBuffer) -> Result<Vec<u8>, Status> {
        self.cdm.decrypt(input_buffer)
    }

    // TODO: delete this and outsource timer management to library users?
    pub fn update(&mut self) {
        for timer in self.host.timer_iter() {
            self.cdm.timer_expired(timer);
        }
    }
}

#[test]
fn test_widevine_api() {
    WidevineAPI::initialize().unwrap();
}

#[tokio::test]
async fn test_cdm_initialization() {
    let mut api = WidevineAPI::initialize().unwrap();
    let result = api.initialize_cdm().await;
    assert!(result.is_ok())
}
