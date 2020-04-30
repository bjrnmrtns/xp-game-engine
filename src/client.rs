use crate::commands::Command;
use serde::{Serialize, Deserialize};

pub trait Sender {
    fn send(&mut self, commands: &[(u64, Vec<Command>)]);
}

pub trait Receiver {
    fn receive(&mut self, from_frame_nr: u64, to_frame_nr: u64) -> Vec<(u64, Vec<Command>)>;
}

pub struct NullSender;

impl Sender for NullSender {
    fn send(&mut self, _ : &[(u64, Vec<Command>)]) {
    }
}

impl NullSender {
    pub fn new() -> NullSender {
        NullSender { }
    }
}

pub struct NullReceiver;

impl Receiver for NullReceiver {
    fn receive(&mut self, from_frame_nr: u64, to_frame_nr: u64) -> Vec<(u64, Vec<Command>)> {
        Vec::new()
    }
}

impl NullReceiver {
    pub fn new() -> NullReceiver {
        NullReceiver { }
    }
}

pub fn send(sender: &mut Sender, commands: &[(u64, Vec<Command>)]) {
    sender.send(commands)
}

pub fn receive(receiver: &mut Receiver, from_frame_nr: u64, to_frame_nr: u64) -> Vec<(u64, Vec<Command>)> {
    receiver.receive(from_frame_nr, to_frame_nr)
}
