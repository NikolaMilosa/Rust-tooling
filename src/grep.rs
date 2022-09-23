use std::fs;

use crate::basic::{self, BuildableCommand, GenericCommand, HelpableCommand};

pub struct GrepCommand {
    pattern: String,
    path: String,
    match_casing: bool,
}

impl basic::GenericCommand for GrepCommand {
    fn run(&self) -> Result<(), &'static str> {
        let contents = match fs::read_to_string(&self.path) {
            Ok(content) => content,
            Err(_) => return Err("Error reading file"),
        };

        contents
            .lines()
            .filter(|line| {
                if self.match_casing {
                    return line.contains(&self.pattern);
                }

                line.to_lowercase()
                    .contains(&self.pattern.as_str().to_lowercase())
            })
            .for_each(|item| println!("{}", item));

        Ok(())
    }
}

impl BuildableCommand for GrepCommand {
    fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Box<dyn GenericCommand>, &'static str> {
        let pattern = match args.next() {
            Some(pattern) => pattern,
            None => return Err("Pattern not found"),
        };

        let path = match args.next() {
            Some(path) => path,
            None => return Err("Path param not found"),
        };

        let match_casing = match args.next() {
            Some(value) => {
                let mut var = false;
                if value == "true" {
                    var = true;
                }
                var
            }
            None => false,
        };

        Ok(Box::new(GrepCommand {
            pattern,
            path,
            match_casing,
        }))
    }
}

impl HelpableCommand for GrepCommand {
    fn help() {
        println!("grep is a small tool developed for better searching of documents.");
        println!();
        println!("It accepts three parameters.");
        println!("\t[Mandatory] pattern");
        println!("\t[Mandatory] path");
        println!("\t[Optional] match_casing (true only if value 'true' inputed");
        println!();
        println!("Example use: ");
        println!("\tcargo run -- grep dep Cargo.toml true");
    }
}
