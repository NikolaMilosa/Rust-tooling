use std::fs::File;

use crate::basic::{BuildableCommand, GenericCommand, HelpableCommand};

pub struct TouchCommand {
    path: String,
}

impl GenericCommand for TouchCommand {
    fn run(&self) -> Result<(), &'static str> {
        let path = std::path::Path::new(&self.path);
        let prefix = match path.parent() {
            Some(prefix) => prefix,
            None => return Err("Not a valid prefix path"),
        };
        match std::fs::create_dir_all(prefix) {
            Ok(_) => {}
            Err(_) => return Err("Cannot create the parent dirs"),
        };
        let _ = match File::create(&self.path) {
            Ok(file) => file,
            Err(_) => return Err("Cannot create file on specified path!"),
        };

        Ok(())
    }
}

impl BuildableCommand for TouchCommand {
    fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Box<dyn GenericCommand>, &'static str> {
        let path = match args.next() {
            Some(path) => path,
            None => return Err("Path argument not found!"),
        };

        Ok(Box::new(TouchCommand { path }))
    }
}

impl HelpableCommand for TouchCommand {
    fn help() {
        println!("Touch command to create files.");
        println!();
        println!("It accepts one parameters.");
        println!("\t[Mandatory] path");
        println!();
        println!("Example use: ");
        println!("\tcargo run -- touch path/to/file.ext");
    }
}
