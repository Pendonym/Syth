use std::error::Error;

pub fn send_heartbeat(
    port: &u16,
    max_players: &u32,
    server_name: &str,
    public: &bool,
) -> Result<(), Box<dyn Error>> {
    let resp = reqwest::blocking::get(format!("https://www.classicube.net/server/heartbeat?port={}&max={}&name={}&public={}&version=7&salt=h9ha2298afhui298&users=0",
    port, max_players, server_name, public))?.text()?;
    println!("{:#?}", resp);
    Ok(())
}
