use crate::Data;

pub async fn connect_rcon(data: &Data) -> Result<rcon::Connection<tokio::net::TcpStream>, String> {
  match 
    rcon::Connection::connect(&data.rcon_addr, &data.rcon_pw).await {
        Ok(resp) => Ok(resp),
        Err(_) => {
          Err("Unable to connect to gameserver via RCON. Server rebooting? ask nari.".into())
        },
    }
}