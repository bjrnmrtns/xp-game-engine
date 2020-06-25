use crate::commands::Command;

pub trait Sender {
    fn send(&mut self, commands: &[(u64, Vec<Command>)]);
}

pub trait Receiver {
    fn receive(&mut self, to_frame_nr: u64) -> Vec<(u64, Vec<Command>)>;
}

pub struct NullSender;

impl Sender for NullSender {
    fn send(&mut self, _: &[(u64, Vec<Command>)]) {
    }
}

impl NullSender {
    pub fn new() -> NullSender {
        NullSender { }
    }
}

pub struct NullReceiver;

impl Receiver for NullReceiver {
    fn receive(&mut self, _: u64) -> Vec<(u64, Vec<Command>)> {
        Vec::new()
    }
}

impl NullReceiver {
    pub fn new() -> NullReceiver {
        NullReceiver { }
    }
}

pub fn send(sender: &mut dyn Sender, commands: &[(u64, Vec<Command>)]) {
    sender.send(commands)
}

pub fn receive(receiver: &mut dyn Receiver, to_frame_nr: u64) -> Vec<(u64, Vec<Command>)> {
    receiver.receive(to_frame_nr)
}
