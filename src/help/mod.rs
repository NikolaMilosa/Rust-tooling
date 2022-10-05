use std::env::Args;

use crate::basic::GenericCommand;

pub struct HelpCommand {
    command: String,
    available_commands: Vec<(
        &'static str,
        fn(),
        fn(Args) -> Result<Box<dyn GenericCommand>, &'static str>,
    )>,
}

impl HelpCommand {
    pub fn new(
        mut args: Args,
        available_commands: Vec<(
            &'static str,
            fn(),
            fn(Args) -> Result<Box<dyn GenericCommand>, &'static str>,
        )>,
    ) -> Result<Box<dyn GenericCommand>, &'static str> {
        let command = match args.next() {
            Some(command) => command,
            None => return Err("Missing command for the help command"),
        };

        return Ok(Box::new(HelpCommand {
            command: command,
            available_commands: available_commands,
        }));
    }
}

impl GenericCommand for HelpCommand {
    fn run(&self) -> Result<(), &'static str> {
        for command in &self.available_commands {
            if command.0 == &self.command {
                command.1();
                return Ok(());
            }
        }

        return Err("Unknown command");
    }

    fn build(_args: Args) -> Result<Box<dyn GenericCommand>, &'static str>
    where
        Self: Sized,
    {
        panic!("Not implemented!")
    }

    fn help()
    where
        Self: Sized,
    {
        println!("help command for the cli.");
        println!();
        println!("It accepts one parameters.");
        println!("\t[Mandatory] command");
        println!();
        println!("Example use: ");
        println!("\tcargo run -- help grep");
    }
}
