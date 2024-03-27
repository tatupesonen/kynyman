use poise::serenity_prelude::{
    self, ComponentInteractionCollector, CreateActionRow, CreateButton, CreateSelectMenu, Message,
};
use regex::Regex;

use crate::rcon::{self, run_workshop_map};
use crate::{Context, Data, Error};

const DEATHMATCH_CMD: &'static str = "game_alias deathmatch; mp_restartgame 5; mp_roundtime 600";
const CUSTOM_CMD: &'static str = "game_alias custom; mp_maxrounds 30; mp_restartgame 5;";
const WINGMAN_CMD: &'static str = "game_alias wingman; mp_maxrounds 30; mp_restartgame 5;";
const CASUAL_CMD: &'static str = "game_alias casual; mp_maxrounds 30; mp_restartgame 5;";
const COMPETITIVE_CMD: &'static str = "game_alias competitive; mp_maxrounds 30; mp_restartgame 5;";

const ROTATE_MAPS: &'static str = "mp_match_end_changelevel 1; mp_match_end_restart 0;";

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
    }
    .await?;
    Ok(())
}

/// Host a workshop map
#[poise::command(slash_command)]
pub async fn workshop_map(
    ctx: Context<'_>,
    #[description = "ID of the workshop map to run"] map: String,
) -> Result<(), Error> {
    let resp = run_workshop_map(ctx.data(), &map).await;
    match resp {
        Ok(resp) => ctx.reply(format!("```{resp}```")),
        Err(_) => ctx.reply(format!("Unable to set server map to {map}")),
    }
    .await?;
    Ok(())
}

/// Host workshop collection
#[poise::command(slash_command)]
pub async fn workshop_collection(
    ctx: Context<'_>,
    #[description = "ID of the workshop collection to run"] collection: String,
) -> Result<(), Error> {
    let mut rcon = rcon::connect_rcon(ctx.data()).await?;
    let collection = collection.trim();
    let cmd = format!("host_workshop_collection {collection}");
    let resp = rcon.cmd(&cmd).await;
    match resp {
        Ok(resp) => ctx.reply(format!("```{resp}```")),
        Err(_) => ctx.reply(format!(
            "Unable to set server map collection to {collection}"
        )),
    }
    .await?;
    Ok(())
}

/// mp_restartgame 1
#[poise::command(slash_command)]
pub async fn restartgame(ctx: Context<'_>) -> Result<(), Error> {
    let mut rcon = rcon::connect_rcon(ctx.data()).await?;
    let resp = rcon.cmd("mp_restartgame 3").await;
    match resp {
        Ok(_) => ctx.reply(format!("Game restarted")),
        Err(_) => ctx.reply(format!("Unable to run mp_restartgame")),
    }
    .await?;
    Ok(())
}

/// Find link of collection or map in message and set that to be the current server map
#[poise::command(context_menu_command = "Set as map", slash_command)]
pub async fn set_map(
    ctx: Context<'_>,
    #[description = "Message to set as the map"] msg: Message,
) -> Result<(), Error> {
    // https://steamcommunity.com/sharedfiles/filedetails/?id=3141537665&searchtext=morass
    let re = Regex::new(r"\?id=(\d+)(?:&|$)").unwrap();
    if let Some(capture) = re.captures(&msg.content) {
        if let Some(numbers) = capture.get(1) {
            run_workshop_map(ctx.data(), numbers.as_str()).await?;
            ctx.reply(format!("Detected map ID: {}, changing.", numbers.as_str())).await?;
        }
    } else {
        ctx.reply("No map or collection found.").await?;
    }
    Ok(())
}
