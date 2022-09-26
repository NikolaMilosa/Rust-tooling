use std::env::Args;

use crate::help::HelpCommand;

pub trait GenericCommand {
    fn run(&self) -> Result<(), &'static str>;

    fn build(args: Args) -> Result<Box<dyn GenericCommand>, &'static str>
    where
        Self: Sized;

    fn help()
    where
        Self: Sized;
}

pub fn build(
    mut args: Args,
    commands: Vec<(
        &'static str,
        fn(),
        fn(Args) -> Result<Box<dyn GenericCommand>, &'static str>,
    )>,
) -> Result<Box<dyn GenericCommand>, &'static str> {
    args.next();

    let spawned_command = match args.next() {
        Some(command) => command,
        None => return Err("Command not found"),
    };

    let lower_command = spawned_command.as_str().to_lowercase();

    if lower_command == "help" {
        let command = match args.next() {
            Some(command) => command,
            None => return Err("Missing command for the help command"),
        };
        return Ok(Box::new(HelpCommand {
            command: command,
            available_commands: commands,
        }));
    }

    for command in commands {
        if command.0 == lower_command.as_str() {
            return command.2(args);
        }
    }

    return Err("Command not found");
}
