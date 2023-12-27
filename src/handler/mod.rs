mod music;
use serenity::async_trait;
use serenity::model::{
    application::{
        command::{Command, CommandOptionType},
        interaction::Interaction,
    },
    gateway::Ready,
};
use serenity::prelude::*;

async fn register_commands(ctx: &Context) -> Result<(), serenity::Error> {
    Command::create_global_application_command(&ctx.http, |command| {
        command
            .name("music")
            .description("Play ANSiMusic")
            .create_option(|option| {
                option
                    .name("sequence")
                    .kind(CommandOptionType::String)
                    .description("ANSI Music codes")
                    .required(true)
            })
    })
    .await?;
    Command::create_global_application_command(&ctx.http, |command| {
        command.name("about").description("About ANSiMusic")
    })
    .await?;
    Ok(())
}

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(cmd) = interaction {
            match cmd.data.name.as_str() {
                "about" => music::about_command(ctx, cmd).await,
                "music" => music::music_command(ctx, cmd).await,
                _ => {}
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        if let Err(error) = register_commands(&ctx).await {
            eprintln!("{error}");
        }
    }
}
