use crate::commands::Command;
use crate::counter::FrameCounter;

pub trait Client {
    fn send(&mut self, commands: &[Command]);
    fn receive(&mut self) -> Vec<(u64, Vec<Command>)>;
}

// Local client is server and client in one (loopback mode for local "networking")
pub struct LocalClient {
    framecounter: FrameCounter,
    client_queue: Vec<Command>,
    server_queue: Vec<(u64, Vec<Command>)>,
    last_merging_in_frame: u64,
}

impl LocalClient {
    pub fn new() -> LocalClient {
        LocalClient {
            framecounter: FrameCounter::new(60),
            client_queue: Vec::new(),
            server_queue: Vec::new(),
            last_merging_in_frame: 0,
        }
    }

    fn merge_to_server_queue(&mut self) {
        if self.framecounter.count() > self.last_merging_in_frame {
            self.server_queue.push((self.framecounter.count(), self.client_queue.clone()));
            self.client_queue.clear();
            self.last_merging_in_frame = self.framecounter.count();
        }
    }
}

impl Client for LocalClient {
    fn send(&mut self, commands: &[Command]) {
        self.framecounter.run();
        self.client_queue.append(commands.to_vec().as_mut());
        self.merge_to_server_queue();
    }

    fn receive(&mut self) -> Vec<(u64, Vec<Command>)> {
        let ret = self.server_queue.clone();
        self.server_queue.clear();
        ret
    }
}
