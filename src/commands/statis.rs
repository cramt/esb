use std::ops::Deref;

use once_cell::sync::Lazy;
use serenity::model::interactions::application_command::{
    ApplicationCommandInteractionDataOption, ApplicationCommandInteractionDataOptionValue,
    ApplicationCommandOptionType,
};

use crate::{commands::CommandOption, ip::get_ip, mc::get_status};

use super::Command;

pub fn all_commands() -> &'static Vec<Command> {
    async fn mc_status(_: Vec<ApplicationCommandInteractionDataOption>) -> String {
        let mc_status = get_status().await;
        match mc_status {
            Ok(status) => {
                let players = status.players.sample;
                let result = players
                    .into_iter()
                    .map(|player| player.name)
                    .collect::<Vec<_>>()
                    .join(", ");
                if result.is_empty() {
                    "Server is empty".to_string()
                } else {
                    format!("Players: {}", result)
                }
            }
            Err(error) => match error {
                crate::mc::McError::TransportError(_) => "Could not conncet to server".to_string(),
                crate::mc::McError::McWriteError(write_error) => {
                    format!("{:?}", write_error)
                }
                crate::mc::McError::McReadError(read_error) => {
                    format!("{:?}", read_error)
                }
                crate::mc::McError::NotFound => "Status not found?".to_string(),
                crate::mc::McError::NotRunning => "Server not running".to_string(),
            },
        }
    }
    async fn ping(_: Vec<ApplicationCommandInteractionDataOption>) -> String {
        "Hey, I'm alive!".to_string()
    }
    async fn id(args: Vec<ApplicationCommandInteractionDataOption>) -> String {
        let options = args
            .get(0)
            .expect("Expected user option")
            .resolved
            .as_ref()
            .expect("Expected user object");
        if let ApplicationCommandInteractionDataOptionValue::User(user, _member) = options {
            format!("{}'s id is {}", user.tag(), user.id)
        } else {
            "Please provide a valid user".to_string()
        }
    }
    async fn ip(_: Vec<ApplicationCommandInteractionDataOption>) -> String {
        match get_ip().await {
            Some(ip) => {
                format!("The server ip is: {}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3])
            }
            None => "Couldn't fetch the ip".to_string(),
        }
    }
    static COMMANDS: Lazy<Vec<Command>> = Lazy::new(|| {
        vec![
            Command::new(
                "mc_status",
                "Checks the status of the minecraft server",
                vec![],
                &mc_status,
            ),
            Command::new("ping", "A ping command", vec![], &ping),
            Command::new(
                "id",
                "Get a user id",
                vec![CommandOption::new(
                    "id",
                    "The user to lookup",
                    ApplicationCommandOptionType::User,
                    true,
                )],
                &id,
            ),
            Command::new("ip", "Gets the server's IP.", vec![], &ip),
        ]
    });
    COMMANDS.deref()
}
