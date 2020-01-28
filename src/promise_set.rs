use crate::types::Exception;
use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::Mutex;
use std::task::{Context, Poll, Waker};

#[derive(Clone, Debug)]
pub enum PromiseResult {
    Resolved(PromiseResultData),
    Rejected(RejectionInfo),
}

impl PromiseResult {
    pub fn as_result(self) -> Result<PromiseResultData, RejectionInfo> {
        match self {
            PromiseResult::Resolved(data) => Ok(data),
            PromiseResult::Rejected(info) => Err(info),
        }
    }
}

#[derive(Clone, Debug)]
pub enum PromiseResultData {
    None,
    Initialized(bool),
    NewSession(String),
}

#[derive(Clone, Debug)]
pub struct RejectionInfo {
    pub exception: Exception,
    pub system_code: u32,
    pub error_message: String,
}

pub const INITIALIZED_PROMISE_ID: usize = 0;

pub struct PromiseSet {
    next: usize,
    stored: HashSet<usize>,
}

// TODO: starting at one for init promise, kinda hackish
impl Default for PromiseSet {
    fn default() -> Self {
        Self {
            next: 1,
            stored: HashSet::new(),
        }
    }
}

impl PromiseSet {
    pub fn create(&mut self) -> usize {
        let id = self.next;
        self.stored.insert(id);
        self.next += 1;
        id
    }

    pub fn pop(&mut self, id: usize) -> bool {
        self.stored.remove(&id)
    }
}

#[derive(Default)]
pub struct PromiseManager {
    pub finished_promises: HashMap<usize, PromiseResult>,
    pub on_finished_promises: HashMap<usize, Waker>,
}

impl PromiseManager {
    pub fn wake(&mut self, id: usize) {
        if let Some(waker) = self.on_finished_promises.remove(&id) {
            waker.wake();
        }
    }
}

pub struct FuturePromise {
    pub host: Arc<Mutex<PromiseManager>>,
    pub id: usize,
}

impl Future for FuturePromise {
    type Output = PromiseResult;

    fn poll(self: Pin<&mut Self>, context: &mut Context) -> Poll<Self::Output> {
        let mut host = self.host.lock().unwrap();
        if let Some(value) = host.finished_promises.remove(&self.id) {
            Poll::Ready(value)
        } else {
            host.on_finished_promises
                .insert(self.id, context.waker().clone());
            Poll::Pending
        }
    }
}
