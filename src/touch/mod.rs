use std::{env::Args, fs::File};

use crate::basic::GenericCommand;

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

    fn build(mut args: Args) -> Result<Box<dyn GenericCommand>, &'static str>
    where
        Self: Sized,
    {
        let path = match args.next() {
            Some(path) => path,
            None => return Err("Path argument not found!"),
        };

        Ok(Box::new(TouchCommand { path }))
    }

    fn help()
    where
        Self: Sized,
    {
        println!("Touch command to create files.");
        println!();
        println!("It accepts one parameters.");
        println!("\t[Mandatory] path");
        println!();
        println!("Example use: ");
        println!("\tcargo run -- touch path/to/file.ext");
    }
}
