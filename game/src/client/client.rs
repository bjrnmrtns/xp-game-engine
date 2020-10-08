use crate::client::command::FrameCommand;

pub trait Sender {
    fn send(&mut self, commands: &[FrameCommand]);
}

pub trait Receiver {
    fn receive(&mut self, to_frame_nr: u64) -> Vec<FrameCommand>;
}

pub struct NullSender;

impl Sender for NullSender {
    fn send(&mut self, _: &[FrameCommand]) {}
}

impl NullSender {
    pub fn new() -> NullSender {
        NullSender {}
    }
}

pub struct NullReceiver;

impl Receiver for NullReceiver {
    fn receive(&mut self, _to_frame_nr: u64) -> Vec<FrameCommand> {
        Vec::new()
    }
}

impl NullReceiver {
    pub fn new() -> NullReceiver {
        NullReceiver {}
    }
}

pub fn send(sender: &mut dyn Sender, commands: &[FrameCommand]) {
    sender.send(commands)
}

pub fn receive(receiver: &mut dyn Receiver, to_frame_nr: u64) -> Vec<FrameCommand> {
    receiver.receive(to_frame_nr)
}
