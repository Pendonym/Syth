use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use syth_config::Config;

fn main() {
    let config = Config::load();
    let address = format!("{}:{}", config.ip, config.port);

    let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(&stream, &config);
    }
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

fn server_indentification(server_name: &str, motd: &str) -> Vec<u8> {
    let mut buf = Vec::<u8>::with_capacity(131);
    buf.push(0x00); // packet id
    buf.push(7); // protocol ver
    write_string(&mut buf, &server_name);
    write_string(&mut buf, &motd);
    buf.push(0x00);
    buf
}

fn handle_connection(mut stream: &TcpStream, config: &Config) -> std::io::Result<()> {
    let mut buf_reader = BufReader::new(stream.try_clone().unwrap());
    let packet: Vec<u8> = buf_reader.fill_buf().unwrap().to_vec();

    let packet_id = packet[0];
    let protocol_version = packet[1];
    let username = read_string(&packet[2..66]);
    let verification_key = read_string(&packet[66..130]);

    // Player Identification packet
    if packet[0] == 0 {
        println!("Received PI packet: {:?}", packet);
        println!(
            "{}, {}, {}, {}",
            packet_id, protocol_version, username, verification_key
        );

        // Server Identification packet
        let server_id = server_indentification(&config.server_name, &config.motd);
        stream.write_all(&server_id);

        return Ok(());
    }

    // got to change this to stop if the player sends other packets
    // Ping packet
    // loop {
    //     stream.write_all(&[0x01]);
    //     std::thread::sleep(std::time::Duration::from_secs(30));
    // }

    // Level Intialize packet
    stream.write_all(&[0x02]);

    Ok(())
}
