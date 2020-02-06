use std::os::raw::c_void;
use std::sync::mpsc::{channel, Receiver, Sender, TryIter};
use std::thread::{sleep, spawn};
use std::time::Duration;

#[derive(Copy, Clone, Debug)]
pub struct Timer {
    pub delay: u64,
    pub context: *mut c_void,
}

unsafe impl Send for Timer {}

#[derive(Debug)]
pub struct TimerManager {
    receiver: Receiver<Timer>,
    sender: Sender<Timer>,
}

impl Default for TimerManager {
    fn default() -> Self {
        let (sender, receiver) = channel();
        Self { sender, receiver }
    }
}

impl TimerManager {
    pub fn new_timer(&self, delay: u64, context: *mut c_void) {
        let sender = self.sender.clone();
        let timer = Timer { delay, context };

        spawn(move || {
            sleep(Duration::from_millis(timer.delay));
            sender.send(timer)
        });
    }

    pub fn try_iter(&mut self) -> TryIter<Timer> {
        self.receiver.try_iter()
    }
}
