use crate::commands::Command;
use crate::client::{Receiver, Sender, NullSender, NullReceiver};
use std::path::PathBuf;
use crate::packet;

pub struct Replayer {
    reader: Box<std::io::Read>,
    read_state: Vec<(u64, Vec<Command>)>,
}
impl Replayer {
    pub fn new(reader: Box<dyn std::io::Read>) -> Replayer {
        Replayer {
            reader,
            read_state: Vec::new(),
        }
    }
}
impl Receiver for Replayer {
    fn receive(&mut self, to_frame_nr: u64) -> Vec<(u64, Vec<Command>)> {
        while let Ok(Some(packet)) = packet::read(&mut *self.reader) {
            let mut deser: Vec<(u64, Vec<Command>)> = serde_cbor::de::from_slice(packet.as_slice()).unwrap();
            self.read_state.append(&mut deser);
            if self.read_state.iter().any(|c| c.0 >= to_frame_nr - 1) {
                let ret = self.read_state.iter().filter(|c| c.0 < to_frame_nr).map(|c| c.clone()).collect();
                self.read_state.retain(|c| c.0 >= to_frame_nr);
                return ret;
            }
        }
        let ret = self.read_state.clone();
        self.read_state.clear();
        ret
    }
}
pub struct Recorder {
    writer: Box<std::io::Write>,
}
impl Recorder {
    pub fn new(writer: Box<dyn std::io::Write>) -> Recorder {
        Recorder {
            writer,
        }
    }
}
impl Sender for Recorder {
    fn send(&mut self, commands: &[(u64, Vec<Command>)]) {
        packet::write(&mut *self.writer, serde_cbor::ser::to_vec(&commands).unwrap().as_slice());
    }
}

// returns a NullSender if no path is given
pub fn try_create_recorder(recording: Option<PathBuf>) -> Box<Sender> {
    match recording {
        Some(path) => {
            let mut f = Box::new(std::fs::OpenOptions::new().write(true).create(true).open(path).expect("cannot open file for recording"));
            Box::new(Recorder::new(f))
        },
        None => Box::new(NullSender::new()),
    }
}

// returns a NullReceiver if no path is given
pub fn try_create_replayer(replay: Option<PathBuf>) -> Box<Receiver> {
    match replay {
        Some(path) => {
            let mut f = Box::new(std::fs::OpenOptions::new().read(true).open(path).expect("cannot open file for replay"));
            Box::new(Replayer::new(f))
        },
        None => Box::new(NullReceiver::new()),
    }
}
