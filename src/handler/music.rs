use std::borrow::{BorrowMut, Cow};

use crate::ansimusic::{Music, Player, SquareWave};
use serenity::prelude::*;
use serenity::{
    builder::CreateEmbed,
    model::{
        application::interaction::{
            application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
            InteractionResponseType,
        },
        channel::AttachmentType,
    },
};

fn create_ansi_escapes(music: &Music) -> Vec<u8> {
    let mut vec = Vec::new();
    vec.append(b"\x1b[MF ".to_vec().borrow_mut());
    vec.append(&mut music.to_string().as_bytes().to_vec());
    vec.append(b"\x0e".to_vec().borrow_mut());
    vec
}

pub async fn music_command(ctx: Context, cmd: ApplicationCommandInteraction) {
    if let CommandDataOptionValue::String(string) = cmd
        .data
        .options
        .first()
        .expect("option")
        .resolved
        .as_ref()
        .expect("value")
    {
        let music = Music::new(string.as_bytes());
        let mut player = Player::new(SquareWave::new(44800));
        player.play(&music);
        if let Err(error) = cmd
            .create_interaction_response(ctx.http, move |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message
                            .add_file(AttachmentType::Bytes {
                                data: Cow::from(player.wav_16_bytes()),
                                filename: "ANSMUSIC.WAV".to_string(),
                            })
                            .add_file(AttachmentType::Bytes {
                                data: Cow::from(create_ansi_escapes(&music)),
                                filename: "ANSMUSIC.ANS".to_string(),
                            })
                            .embed(|embed: &mut CreateEmbed| {
                                embed.color(0xff0000).description(&music.to_string())
                            })
                    })
            })
            .await
        {
            eprintln!("{error}");
        }
    } else {
        unreachable!("string expected");
    }
}

pub async fn about_command(ctx: Context, cmd: ApplicationCommandInteraction) {
    if let Err(error) = cmd
        .create_interaction_response(ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.embed(|embed: &mut CreateEmbed| {
                        embed
                            .color(0xff0000)
                            .description(include_str!("../about.md"))
                    })
                })
        })
        .await
    {
        eprintln!("{error}");
    }
}
