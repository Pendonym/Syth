pub mod packet_ids {
    pub const IDENTIFICATION: u8 = 0x00;
    pub const PING: u8 = 0x01;
    pub const LEVEL_INIT: u8 = 0x02;
    pub const LEVEL_DATA_CHUNK: u8 = 0x03;
    pub const LEVEL_FINALIZE: u8 = 0x04;
}

fn read_string(b: &[u8]) -> String {
    String::from_utf8_lossy(b).trim_end().to_string()
}

fn write_string(buf: &mut Vec<u8>, string: &str) {
    let mut b = [b' '; 64];
    let n = string.len().min(64);
    b[..n].copy_from_slice(&string.as_bytes()[..n]);
    buf.extend_from_slice(&b);
}

pub fn server_identification(server_name: &str, motd: &str) -> Vec<u8> {
    let mut buf = Vec::<u8>::with_capacity(131);
    buf.push(packet_ids::IDENTIFICATION);
    buf.push(7);
    write_string(&mut buf, &server_name);
    write_string(&mut buf, &motd);
    buf.push(0x00); // op (0x64) or not (0x00)
    buf
}

pub struct PlayerIdentification {
    pub protocol_version: u8,
    pub username: String,
    pub verification_key: String,
}

impl PlayerIdentification {
    pub fn parse(packet: &[u8]) -> Option<Self> {
        if packet.len() < 64 || packet[0] != packet_ids::IDENTIFICATION {
            return None;
        }
        Some(Self {
            protocol_version: packet[1],
            username: read_string(&packet[2..66]),
            verification_key: read_string(&packet[66..130]),
        })
    }
}
