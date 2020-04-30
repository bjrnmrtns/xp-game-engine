use crate::commands::Command;

// Local client is server and client in one (loopback mode for local "networking")
pub struct LocalClient {
    server_queue: Vec<(u64, Vec<Command>)>,
}

impl LocalClient {
    pub fn new() -> LocalClient {
        LocalClient {
            server_queue: Vec::new(),
        }
    }

    pub fn send(&mut self, commands: &[(u64, Vec<Command>)]) {
        self.server_queue.append(commands.to_vec().as_mut());
    }

    pub fn receive(&mut self, frame_nr: u64) ->  Vec<(u64, Vec<Command>)> {
        let ret = self.server_queue.iter().filter(|c| c.0 <= frame_nr).map(|c| c.clone()).collect();
        self.server_queue.retain(|c| c.0 > frame_nr);
        ret
    }
}
