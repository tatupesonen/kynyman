use poise::serenity_prelude::{self as serenity, GuildId, RoleId};

#[derive(Debug)]
struct Data {
    rcon_pw: String,
    rcon_addr: String,
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
mod commands;
mod rcon;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    let token = std::env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN.");
    let RCON_ADDR = std::env::var("RCON_ADDR").expect("Missing RCON_ADDR.");
    let RCON_PW = std::env::var("RCON_PW").expect("Missing RCON_PW.");
    let _ = GuildId::new(std::env::var("GUILD_ID").expect("Missing GUILD_ID.").parse().unwrap());
    let _ = RoleId::new(std::env::var("ROLE_ID").expect("Missing ROLE_ID.").parse().unwrap());
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::execute(),
                commands::gametype(),
                commands::workshop_map(),
            ],
            command_check: Some(|ctx| {
                Box::pin(async move {
                  let guild_id = GuildId::new(std::env::var("GUILD_ID").expect("Missing GUILD_ID.").parse().unwrap());
                  let role_id = RoleId::new(std::env::var("ROLE_ID").expect("Missing ROLE_ID.").parse().unwrap());
                  let author_has_role = ctx.author().has_role(ctx, &guild_id, &role_id).await?;
                  if author_has_role {
                    Ok(author_has_role)
                  } else {
                    ctx.reply("You don't have the role you dirty fucker").await?;
                    Ok(author_has_role)
                  }
                })
            }),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    rcon_addr: RCON_ADDR,
                    rcon_pw: RCON_PW,
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(&token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
