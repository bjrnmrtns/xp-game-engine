use crate::client::client::{Receiver, Sender};
use crate::client::command::FrameCommand;

// Local client is server and client in one (loopback mode for local "networking")
pub struct LocalClient {
    server_queue: Vec<FrameCommand>,
}

impl LocalClient {
    pub fn new() -> LocalClient {
        LocalClient {
            server_queue: Vec::new(),
        }
    }
}

impl Receiver for LocalClient {
    fn receive(&mut self, to_frame_nr: u64) -> Vec<FrameCommand> {
        let ret = self
            .server_queue
            .iter()
            .filter(|c| c.frame < to_frame_nr)
            .map(|c| c.clone())
            .collect();
        self.server_queue.retain(|c| c.frame >= to_frame_nr);
        ret
    }
}

impl Sender for LocalClient {
    fn send(&mut self, commands: &[FrameCommand]) {
        self.server_queue.extend_from_slice(commands);
    }
}
