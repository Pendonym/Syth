use std::error::Error;

pub fn send_heartbeat() -> Result<(), Box<dyn Error>> {
    let resp = reqwest::blocking::get("https://www.classicube.net/server/heartbeat?port=25565&max=12&name=Test&public=True&version=7&salt=h9ha2298afhui298&users=0")?.text()?;
    println!("{:#?}", resp);
    Ok(())
}
