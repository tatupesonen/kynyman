use crate::{Context, Error};
use crate::rcon;

const DEATHMATCH_CMD: &'static str = "game_alias deathmatch; mp_restartgame 5; mp_roundtime 600";
const CUSTOM_CMD: &'static str = "game_alias custom; mp_restartgame 5";
const WINGMAN_CMD: &'static str = "game_alias wingman; mp_restartgame 5";
const CASUAL_CMD: &'static str = "game_alias casual; mp_restartgame 5";
const COMPETITIVE_CMD: &'static str = "game_alias competitive; mp_restartgame 5";


#[derive(Debug, poise::ChoiceParameter)]
pub enum GameType {
    Deathmatch,
    Casual,
    Competitive,
    Wingman,
    Custom,
}

impl GameType {
  fn command(&self) -> &'static str {
    match *self {
        GameType::Deathmatch => DEATHMATCH_CMD,
        GameType::Casual => CASUAL_CMD,
        GameType::Competitive => COMPETITIVE_CMD,
        GameType::Wingman => WINGMAN_CMD,
        GameType::Custom => CUSTOM_CMD,
    }
  }
}


/// Executes a command on the server
#[poise::command(slash_command, prefix_command)]
pub async fn execute(
    ctx: Context<'_>,
    #[description = "Command to execute"] cmd: String,
) -> Result<(), Error> {
    let mut rcon = rcon::connect_rcon(ctx.data()).await?;
    let resp = rcon.cmd(&cmd).await;
    match resp {
        Ok(resp) => {
            let response = if resp == "" {
                format!("Executed `{cmd}`")
            } else {
                format!("Executed `{cmd}`\n```{resp}```")
            };
            ctx.reply(response).await?;
        }
        Err(e) => {
            ctx.reply(format!("Unable to execute command: {e}")).await?;
        }
    }
    // explicit drop here coz why not
    drop(rcon);

    Ok(())
}

/// Set game type to one of the given choices
#[poise::command(slash_command)]
pub async fn gametype(
    ctx: Context<'_>,
    #[description = "Game type"]
    #[rename = "type"]
    game_type: GameType,
) -> Result<(), Error> {
    let mut rcon = rcon::connect_rcon(ctx.data()).await?;

    let cmd = game_type.command();
    let resp = rcon.cmd(cmd).await;
    match resp {
      Ok(_) => ctx.reply(format!("Set game mode to {game_type:?}")),
      Err(_) => ctx.reply(format!("Unable to set game mode to {game_type:?}")),
    }.await?;
    Ok(())
}

/// Host a workshop map
#[poise::command(slash_command)]
pub async fn workshop_map(
    ctx: Context<'_>,
    #[description = "ID of the workshop map to run"]
    map: String,
) -> Result<(), Error> {
    let mut rcon = rcon::connect_rcon(ctx.data()).await?;
    let map = map.trim();
    let cmd = format!("host_workshop_map {map}");
    let resp = rcon.cmd(&cmd).await;
    match resp {
      Ok(_) => ctx.reply(format!("Set server map to {map}")),
      Err(_) => ctx.reply(format!("Unable to set server map to {map}")),
    }.await?;
    Ok(())
}

