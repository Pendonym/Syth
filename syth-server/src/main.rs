mod connection;
mod heartbeat;
mod protocol;

use std::net::TcpListener;
use syth_config::Config;

fn main() {
    let config = Config::load();
    let address = format!("{}:{}", config.ip, config.port);
    let listener = TcpListener::bind(address).unwrap();

    heartbeat::send_heartbeat(
        &config.port,
        &config.max_players,
        &config.server_name,
        &config.public,
    );

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = connection::handle_connection(stream, &config) {
                    eprintln!("An error occured: {}", e)
                }
            }
            Err(e) => eprintln!("An error occured: {}", e),
        }
    }
}
