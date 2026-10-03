use crate::protocol::{self, PlayerIdentification};

use std::io::{Read, Write};
use std::net::TcpStream;
use syth_config::Config;
use syth_world;

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

    stream.write_all(&protocol::server_identification(
        &config.server_name,
        &config.motd,
    ));

    stream.write_all(&[protocol::packet_ids::LEVEL_INIT]);
    stream.write_all(&[protocol::packet_ids::LEVEL_DATA_CHUNK]);
    stream.write_all(&syth_world::level_finalize(
        protocol::packet_ids::LEVEL_FINALIZE,
        16,
        16,
        16,
    ));

    // got to change this to stop if the player sends other packets
    // Ping packet
    loop {
        stream.write_all(&[0x01]);
        std::thread::sleep(std::time::Duration::from_secs(30));
    }
    Ok(())
}
