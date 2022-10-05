use std::{
    env::{self, Args},
    io::Read,
};

use serde_json::Value;

use crate::basic::GenericCommand;

pub struct YtSearchCommand {
    query: String,
    api_key: String,
}

impl GenericCommand for YtSearchCommand {
    fn run(&self) -> Result<(), &'static str> {
        let url = format!(
            "https://www.googleapis.com/youtube/v3/search?key={}&maxResults=1&part=snippet&type=video&q={}",
            &self.api_key, &self.query
        );

        let mut res = match reqwest::get(&url) {
            Ok(res) => res,
            Err(_) => return Err("Couldn't fetch from Youtube API"),
        };
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();

        let v: Value = serde_json::from_str(&body).unwrap();

        let video_id = v["items"][0]["id"]["videoId"].as_str().unwrap();

        let vid_url = format!("https://youtube.com/watch?v={}", video_id);

        match webbrowser::open(&vid_url) {
            Ok(_) => return Ok(()),
            Err(_) => return Err("Couldn't start the video in browser"),
        };
    }

    fn build(mut args: Args) -> Result<Box<dyn GenericCommand>, &'static str>
    where
        Self: Sized,
    {
        let api_key = match env::var("YOUTUBE_API") {
            Ok(api_key) => api_key,
            Err(_) => return Err("Api key for Youtube API not found"),
        };

        let query = match args.next() {
            Some(query) => query,
            None => return Err("Query argument not found"),
        };

        Ok(Box::new(YtSearchCommand { query, api_key }))
    }

    fn help()
    where
        Self: Sized,
    {
        println!("Youtube video launcher in default browser.");
        println!();
        println!("It accepts one parameters.");
        println!("\t[Mandatory] query");
        println!();
        println!("Example use: ");
        println!("\tcargo run -- yt test");
    }
}
