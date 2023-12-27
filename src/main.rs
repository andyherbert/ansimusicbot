mod ansimusic;
mod handler;
use handler::Handler;
use serenity::prelude::*;

#[tokio::main]
async fn main() {
    let token = include_str!("../.token").trim();
    let mut client = Client::builder(token, GatewayIntents::non_privileged())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(err) = client.start().await {
        println!("Client error: {err}");
    }
}
