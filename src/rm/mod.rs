use std::{env::Args, fs};

use crate::basic::GenericCommand;

pub struct RmCommand {
    path: String,
}

impl GenericCommand for RmCommand {
    fn run(&self) -> Result<(), &'static str> {
        let metadata = match fs::metadata(&self.path) {
            Ok(meta) => meta,
            Err(_) => return Err("Couldn't load metadata for path"),
        };

        if metadata.is_dir() {
            match fs::remove_dir_all(&self.path) {
                Ok(_) => return Ok(()),
                Err(_) => return Err("Couldn't delete directory"),
            }
        } else if metadata.is_file() {
            match fs::remove_file(&self.path) {
                Ok(_) => return Ok(()),
                Err(_) => return Err("Couldn't delete file"),
            }
        }

        Err("Path is not a directory nur is it a file")
    }

    fn build(mut args: Args) -> Result<Box<dyn GenericCommand>, &'static str>
    where
        Self: Sized,
    {
        let path = match args.next() {
            Some(path) => path,
            None => return Err("Path argument not found!"),
        };

        Ok(Box::new(RmCommand { path }))
    }

    fn help()
    where
        Self: Sized,
    {
        println!("rm command to delete files and directories.");
        println!();
        println!("It accepts one parameters.");
        println!("\t[Mandatory] path");
        println!();
        println!("Example use: ");
        println!("\tcargo run -- rm path/to/file.ext");
    }
}
