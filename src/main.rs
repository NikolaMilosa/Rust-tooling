use std::{
    env::{self, Args},
    process,
};

use basic::GenericCommand;
use gif_download::GifSearchCommand;
use grep::GrepCommand;
use rm::RmCommand;
use touch::TouchCommand;
use yt_search::YtSearchCommand;

mod basic;
mod gif_download;
mod grep;
mod help;
mod rm;
mod touch;
mod yt_search;

fn main() {
    let available_commands: Vec<(
        &str,
        fn(),
        fn(Args) -> Result<Box<dyn GenericCommand>, &'static str>,
    )> = vec![
        ("gifsrc", GifSearchCommand::help, GifSearchCommand::build),
        ("grep", GrepCommand::help, GrepCommand::build),
        ("touch", TouchCommand::help, TouchCommand::build),
        ("rm", RmCommand::help, RmCommand::build),
        ("yt", YtSearchCommand::help, YtSearchCommand::build),
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
