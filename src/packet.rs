use byteorder::{ByteOrder};
use std::io;

// packet -> ( size: u32, payload: [u8; size] )

pub struct Writer<'a> {
    writer: &'a mut dyn std::io::Write,
}

pub struct Reader<'a> {
    reader: &'a mut dyn std::io::Read,
}

impl Writer<'_> {
    pub fn new(writer: &mut dyn std::io::Write) -> Writer {
        Writer { writer, }
    }
    // write exactly one packet, change later to write multiple and store state
    // of writing
    pub fn write_packet(&mut self, packet_data: &[u8]) -> io::Result<()> {
        let mut size = [0; 4];
        byteorder::NetworkEndian::write_u32(&mut size, packet_data.len() as u32);
        self.writer.write_all(&size)?;
        self.writer.write_all(&packet_data)?;
        Ok(())
    }
}

impl Reader<'_> {
    pub fn new(reader: &mut dyn std::io::Read) -> Reader {
        Reader { reader, }
    }
    // read exactly one packet, change later into internally read more and store state
    // of reading
    pub fn read_packet(&mut self) -> io::Result<Option<Vec<u8>>> {
        let mut len: [u8; 4] = [0; 4];
        self.reader.read_exact(&mut len)?;
        let len= byteorder::NetworkEndian::read_u32(&mut len) as usize;
        let mut packet_data: Vec<u8> = vec!(0; len);
        self.reader.read_exact(packet_data.as_mut_slice())?;
        Ok(Some(packet_data))
    }
}
