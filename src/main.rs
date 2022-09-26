use std::{env, process};

mod basic;
mod gif_download;
mod grep;
mod help;
mod touch;

fn main() {
    let args = env::args().into_iter();

    let command = basic::build(args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1)
    });

    command.as_ref().run().unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1)
    });
}
