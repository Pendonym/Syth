use crate::protocol::{self, PlayerIdentification};

use std::io::{Read, Write};
use std::net::TcpStream;
use syth_config::Config;

pub fn handle_connection(mut stream: TcpStream, config: &Config) -> std::io::Result<()> {
    let mut packet = [0u8; 131];
    stream.read_exact(&mut packet);

    let Some(player) = PlayerIdentification::parse(&packet) else {
        return Ok(());
    };

    println!(
        "{} connected with protocol {} and verification key {}",
        player.username, player.protocol_version, player.verification_key
    );

    // got to change this to stop if the player sends other packets
    // Ping packet
    // loop {
    //     stream.write_all(&[0x01]);
    //     std::thread::sleep(std::time::Duration::from_secs(30));
    // }

    stream.write_all(&protocol::server_indentification(
        &config.server_name,
        &config.motd,
    ));
    stream.write_all(&[protocol::packet_ids::LEVEL_INIT]); // TODO: real Level Initialize packet

    Ok(())
}
