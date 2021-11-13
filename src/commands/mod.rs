pub mod statis;

use std::{future::Future, pin::Pin};

use serenity::model::interactions::application_command::{
    ApplicationCommandInteractionDataOption, ApplicationCommandOptionType,
};

type CommandFunc = Box<
    dyn Send
        + Sync
        + Fn(
            Vec<ApplicationCommandInteractionDataOption>,
        ) -> Pin<Box<dyn Future<Output = String> + Send>>,
>;

pub struct Command {
    pub name: String,
    pub description: String,
    pub options: Vec<CommandOption>,
    pub func: CommandFunc,
}

impl Command {
    pub fn new<
        R: Future<Output = String> + 'static + Send,
        F: (Fn(Vec<ApplicationCommandInteractionDataOption>) -> R) + 'static + Send + Sync,
        S1: ToString,
        S2: ToString,
    >(
        name: S1,
        description: S2,
        options: Vec<CommandOption>,
        func: &'static F,
    ) -> Self {
        let name = name.to_string();
        let description = description.to_string();
        Self {
            name,
            description,
            options,
            func: Box::new(|x| Box::pin(func(x))),
        }
    }
}

pub struct CommandOption {
    pub name: String,
    pub description: String,
    pub kind: ApplicationCommandOptionType,
    pub required: bool,
}

impl CommandOption {
    pub fn new<S1: ToString, S2: ToString>(
        name: S1,
        description: S2,
        kind: ApplicationCommandOptionType,
        required: bool,
    ) -> Self {
        let name = name.to_string();
        let description = description.to_string();
        Self {
            name,
            description,
            kind,
            required,
        }
    }
}
