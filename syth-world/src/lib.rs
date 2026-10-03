pub fn level_finalize(packet_id: u8, x: u16, y: u16, z: u16) -> Vec<u8> {
    let mut buf = Vec::<u8>::with_capacity(7);
    buf.push(packet_id);
    buf.extend_from_slice(&x.to_be_bytes());
    buf.extend_from_slice(&y.to_be_bytes());
    buf.extend_from_slice(&z.to_be_bytes());
    buf
}
