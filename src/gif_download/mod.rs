use std::{
    env::{self, Args},
    fs::File,
    io::{Read, Write},
};

use serde_json::Value;

use crate::basic::{self, GenericCommand};

pub struct GifSearchCommand {
    term: String,
    size: usize,
    order: usize,
    output_file: String,
    api_key: String,
}

impl basic::GenericCommand for GifSearchCommand {
    fn run(&self) -> Result<(), &'static str> {
        let url = format!(
            "https://tenor.googleapis.com/v2/search?q={}&key={}&client_key={}&limit={}",
            &self.term, &self.api_key, "Rusting", &self.size
        );

        let mut res = match reqwest::get(&url) {
            Ok(res) => res,
            Err(_) => return Err("Couldn't fetch from Tenor API"),
        };
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();

        let v: Value = serde_json::from_str(&body).unwrap();

        let value = v["results"][&self.order]["media_formats"]["gif"]["url"]
            .as_str()
            .unwrap();

        res = match reqwest::get(value) {
            Ok(response) => response,
            Err(_) => return Err("Cannot get the ordered gif!"),
        };

        let data: Result<Vec<_>, _> = res.bytes().collect();
        let data = data.unwrap();

        let path = std::path::Path::new(&self.output_file);
        let prefix = match path.parent() {
            Some(prefix) => prefix,
            None => return Err("Not a valid prefix path"),
        };
        match std::fs::create_dir_all(prefix) {
            Ok(_) => {}
            Err(_) => return Err("Cannot create the parent dirs"),
        };

        let mut file = match File::create(&self.output_file) {
            Ok(file) => file,
            Err(_) => return Err("Cannot create file on specified path"),
        };

        let result_output = file.write_all(&data);
        if let Err(_) = result_output {
            return Err("Couldn't write to output file");
        }

        Ok(())
    }

    fn build(mut args: Args) -> Result<Box<dyn GenericCommand>, &'static str>
    where
        Self: Sized,
    {
        let term = match args.next() {
            Some(term) => term,
            None => return Err("Term argument not found!"),
        };

        let size = match args.next() {
            Some(size) => match size.parse::<usize>() {
                Ok(size) => size,
                Err(_) => return Err("Cannot parse argument size to u8"),
            },
            None => return Err("Size argument not found!"),
        };

        let order = match args.next() {
            Some(order) => match order.parse::<usize>() {
                Ok(order) => {
                    if order >= size {
                        return Err("Order has to be in range [0..size)!");
                    }
                    order
                }
                Err(_) => return Err("Cannot parse argument order to u8"),
            },
            None => return Err("Order argument not found!"),
        };

        let output_file = match args.next() {
            Some(path) => path,
            None => return Err("Output path argument not found!"),
        };

        let api_key = match env::var("GIF_API_KEY") {
            Ok(api_key) => api_key,
            Err(_) => return Err("Api key for Tenor API not found"),
        };

        Ok(Box::new(GifSearchCommand {
            term,
            size,
            order,
            output_file,
            api_key,
        }))
    }

    fn help()
    where
        Self: Sized,
    {
        println!("gif command for searching gifs from Tenor API by term.");
        println!();
        println!("It accepts three parameters.");
        println!("\t[Mandatory] term");
        println!("\t[Mandatory] size");
        println!("\t[Mandatory] order (has to be less than size)");
        println!("\t[Mandatory] output_path");
        println!();
        println!("Example use: ");
        println!("\tcargo run -- gifsrc tree 5 3");
    }
}
