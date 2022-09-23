use crate::{gif_download::GifSearchCommand, grep::GrepCommand, help::HelpCommand};

pub trait GenericCommand {
    fn run(&self) -> Result<(), &'static str>;
}

pub trait BuildableCommand {
    fn build(args: impl Iterator<Item = String>) -> Result<Box<dyn GenericCommand>, &'static str>;
}

pub trait HelpableCommand {
    fn help();
}

pub fn build(
    mut args: impl Iterator<Item = String>,
) -> Result<Box<dyn GenericCommand>, &'static str> {
    args.next();

    let spawned_command = match args.next() {
        Some(command) => command,
        None => return Err("Command not found"),
    };

    let lower_command = spawned_command.as_str().to_lowercase();

    match lower_command.as_str() {
        "help" => return HelpCommand::build(args),
        "grep" => return GrepCommand::build(args),
        "gifsrc" => return GifSearchCommand::build(args),
        _ => return Err("Command not found"),
    };
}
