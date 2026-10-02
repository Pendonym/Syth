use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use syth_config::Config;

fn main() {
    let config = Config::load();
    let address = format!("{}:{}", config.ip, config.port);

    let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(&stream);
    }
}

fn read_string(b: &[u8]) -> String {
    String::from_utf8_lossy(b).trim_end().to_string()
}

fn write_string(buf: &mut Vec<u8>, s: &str) {}

fn handle_connection(stream: &TcpStream) -> bool {
    let mut buf_reader = BufReader::new(stream.try_clone().unwrap());
    let mut buf_writer = BufWriter::new(stream.try_clone().unwrap());
    let packet: Vec<u8> = buf_reader.fill_buf().unwrap().to_vec();

    let packet_id = packet[0];
    let protocol_version = packet[1];
    let username = read_string(&packet[2..65]);
    let verification_key = read_string(&packet[66..130]);
    let unused = packet[130];

    println!("Received packet: {:?}", packet);
    println!(
        "{}, {}, {}, {}, {}",
        packet_id, protocol_version, username, verification_key, unused
    );

    //buf_writer.write(buf);

    return false;
}
