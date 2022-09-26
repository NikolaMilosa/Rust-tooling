use crate::{
    basic::{self, BuildableCommand, GenericCommand, HelpableCommand},
    gif_download::GifSearchCommand,
    grep::GrepCommand,
    touch::TouchCommand,
};

pub struct HelpCommand {
    command: String,
}

impl basic::GenericCommand for HelpCommand {
    fn run(&self) -> Result<(), &'static str> {
        let curr_command_lower = &self.command.as_str().to_lowercase();
        if curr_command_lower == "help" {
            HelpCommand::help();
            return Ok(());
        } else if curr_command_lower == "grep" {
            GrepCommand::help();
            return Ok(());
        } else if curr_command_lower == "gifsrc" {
            GifSearchCommand::help();
            return Ok(());
        } else if curr_command_lower == "touch" {
            TouchCommand::help();
            return Ok(());
        }

        Err("Command not found")
    }
}

impl BuildableCommand for HelpCommand {
    fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Box<dyn GenericCommand>, &'static str> {
        let command = match args.next() {
            Some(command) => command,
            None => return Err("Unknown command"),
        };

        Ok(Box::new(HelpCommand { command }))
    }
}

impl HelpableCommand for HelpCommand {
    fn help() {
        println!("help command for the cli.");
        println!();
        println!("It accepts one parameters.");
        println!("\t[Mandatory] command");
        println!();
        println!("Example use: ");
        println!("\tcargo run -- help grep");
    }
}
