use std::{
    env::{self, Args},
    process,
};

use basic::GenericCommand;
use gif_download::GifSearchCommand;
use grep::GrepCommand;
use touch::TouchCommand;

mod basic;
mod gif_download;
mod grep;
mod help;
mod touch;

fn main() {
    let available_commands: Vec<(
        &str,
        fn(),
        fn(Args) -> Result<Box<dyn GenericCommand>, &'static str>,
    )> = vec![
        ("gifsrc", GifSearchCommand::help, GifSearchCommand::build),
        ("grep", GrepCommand::help, GrepCommand::build),
        ("touch", TouchCommand::help, TouchCommand::build),
    ];

    let args = env::args().into_iter();

    let command = basic::build(args, available_commands).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1)
    });

    command.as_ref().run().unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1)
    });
}
