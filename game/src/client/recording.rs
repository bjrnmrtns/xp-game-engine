use crate::client::client::{NullReceiver, NullSender, Receiver, Sender};
use crate::client::command::FrameCommand;
use crate::client::packet;
use std::path::PathBuf;

pub struct Replayer {
    reader: Box<dyn std::io::Read>,
    read_state: Vec<FrameCommand>,
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
    fn receive(&mut self, to_frame_nr: u64) -> Vec<FrameCommand> {
        loop {
            match packet::read(&mut *self.reader) {
                Ok(Some(packet)) => {
                    let mut deser: Vec<FrameCommand> =
                        serde_cbor::de::from_slice(packet.as_slice()).unwrap();
                    self.read_state.append(&mut deser);
                    if self.read_state.iter().any(|c| c.frame >= to_frame_nr - 1) {
                        let ret = self
                            .read_state
                            .iter()
                            .filter(|c| c.frame < to_frame_nr)
                            .map(|c| c.clone())
                            .collect();
                        self.read_state.retain(|c| c.frame >= to_frame_nr);
                        return ret;
                    }
                }
                _ => {
                    let ret = self.read_state.clone();
                    self.read_state.clear();
                    return ret;
                }
            }
        }
    }
}
pub struct Recorder {
    writer: Box<dyn std::io::Write>,
}
impl Recorder {
    pub fn new(writer: Box<dyn std::io::Write>) -> Recorder {
        Recorder { writer }
    }
}
impl Sender for Recorder {
    fn send(&mut self, commands: &[FrameCommand]) {
        packet::write(
            &mut *self.writer,
            serde_cbor::ser::to_vec(&commands).unwrap().as_slice(),
        )
        .unwrap();
    }
}

// returns a NullSender if no path is given
pub fn try_create_recorder(recording: Option<PathBuf>) -> Box<dyn Sender> {
    match recording {
        Some(path) => {
            let f = Box::new(
                std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(path)
                    .expect("cannot open file for recording"),
            );
            Box::new(Recorder::new(f))
        }
        None => Box::new(NullSender::new()),
    }
}

// returns a NullReceiver if no path is given
pub fn try_create_replayer(replay: Option<PathBuf>) -> Box<dyn Receiver> {
    match replay {
        Some(path) => {
            let f = Box::new(
                std::fs::OpenOptions::new()
                    .read(true)
                    .open(path)
                    .expect("cannot open file for replay"),
            );
            Box::new(Replayer::new(f))
        }
        None => Box::new(NullReceiver::new()),
    }
}
