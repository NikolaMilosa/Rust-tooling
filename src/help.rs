use std::env::Args;

use crate::basic::GenericCommand;

pub struct HelpCommand {
    pub command: String,
    pub available_commands: Vec<(
        &'static str,
        fn(),
        fn(Args) -> Result<Box<dyn GenericCommand>, &'static str>,
    )>,
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
