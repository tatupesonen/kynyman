use crate::Data;

pub async fn connect_rcon(data: &Data) -> Result<rcon::Connection<tokio::net::TcpStream>, String> {
    match rcon::Connection::connect(&data.rcon_addr, &data.rcon_pw).await {
        Ok(resp) => Ok(resp),
        Err(_) => {
            Err("Unable to connect to gameserver via RCON. Server rebooting? ask nari.".into())
        }
    }
}

pub async fn get_maplist(data: &Data) -> Result<Vec<String>, String> {
    let mut rcon = connect_rcon(&data).await?;
    let map_str = rcon.cmd("maps *").await.map_err(|_| "Unable to get maps")?;

    let o = map_str
        .lines()
        .map(|line| line.trim().to_owned())
        .filter(|line| {
            ["ar_", "de_", "cs_"]
                .iter()
                .any(|s| line.starts_with(*s))
        })
        .collect::<Vec<String>>();

    Ok(o)
}

pub async fn run_workshop_map(data: &Data, map_id: &str) -> Result<String, String> {
    let mut rcon = connect_rcon(data).await?;
    let collection = map_id.trim();
    let cmd = format!("host_workshop_map {collection}");
    let resp = rcon.cmd(&cmd).await.map_err(|_| "Unable to set map")?;
    Ok(resp)
}