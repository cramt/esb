use futures::future::join_all;
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        id::GuildId,
        interactions::{
            application_command::ApplicationCommand, Interaction, InteractionResponseType,
        },
        prelude::{Ready, User},
    },
};

use crate::{commands::statis::all_commands, constants::users};

fn authorized(user: &User) -> bool {
    users().contains(&user.id.0)
}

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let user = &command.user;
            let content = if !authorized(user) {
                "User not authorized".to_string()
            } else {
                match all_commands()
                    .iter()
                    .find(|c| c.name.as_str() == command.data.name.as_str())
                {
                    Some(c) => c.func.as_ref()(command.data.options.clone()).await,
                    None => "not implemented".to_string(),
                }
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let commands = ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
            all_commands().iter().for_each(|command| {
                commands.create_application_command(|c| {
                    c.name(&command.name).description(&command.description);
                    command.options.iter().for_each(|option| {
                        c.create_option(|o| {
                            o.name(&option.name)
                                .description(&option.description)
                                .kind(option.kind)
                                .required(option.required)
                        });
                    });
                    c
                });
            });
            commands
        })
        .await;

        println!(
            "I now have the following global slash commands: {:?}",
            commands
        );

        let guild = GuildId(337731942399082498);
        let guild_commands = join_all(
            all_commands()
                .iter()
                .map(|command| {
                    guild.create_application_command(&ctx.http, |c| {
                        c.name(&command.name).description(&command.description);
                        command.options.iter().for_each(|option| {
                            c.create_option(|o| {
                                o.name(&option.name)
                                    .description(&option.description)
                                    .kind(option.kind)
                                    .required(option.required)
                            });
                        });
                        c
                    })
                })
                .collect::<Vec<_>>(),
        )
        .await;

        let guild_commands = guild_commands
            .into_iter()
            .map(|command| command.expect("failed to create command"))
            .collect::<Vec<_>>();

        println!("created application commands {:?}", guild_commands);
    }
}
