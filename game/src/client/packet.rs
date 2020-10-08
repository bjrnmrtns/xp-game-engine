use byteorder::ByteOrder;

// packet -> ( size: u32, payload: [u8; size] )

// write exactly one packet, change later to write multiple and store state
// of writing
pub fn write(writer: &mut dyn std::io::Write, packet_data: &[u8]) -> std::io::Result<()> {
    let mut size = [0; 4];
    byteorder::NetworkEndian::write_u32(&mut size, packet_data.len() as u32);
    writer.write_all(&size)?;
    writer.write_all(&packet_data)?;
    Ok(())
}

// read exactly one packet, change later into internally read more and store state
// of reading
pub fn read(reader: &mut dyn std::io::Read) -> std::io::Result<Option<Vec<u8>>> {
    let mut len: [u8; 4] = [0; 4];
    reader.read_exact(&mut len)?;
    let len = byteorder::NetworkEndian::read_u32(&mut len) as usize;
    let mut packet_data: Vec<u8> = vec![0; len];
    reader.read_exact(packet_data.as_mut_slice())?;
    Ok(Some(packet_data))
}
