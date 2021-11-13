mod commands;
mod constants;
mod handler;
mod ip;
mod mc;
#[cfg(test)]
mod test_helper;

use constants::{application_id, discord_secret};
use handler::Handler;

use serenity::Client;

#[tokio::main]
async fn main() {
    let token = discord_secret();

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .application_id(application_id())
        .await
        .expect("something went wrong");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
