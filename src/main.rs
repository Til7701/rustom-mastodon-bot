use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, thread};
extern crate chrono;
extern crate timer;

mod config;
use crate::config::Config;
use chrono::DateTime;
mod mastodon;
use crate::mastodon::MyMastodonClient;
mod sentence;
use crate::sentence::SentenceProvider;
mod io;
mod pattern_parser;

fn main() {
    let timer = timer::Timer::new();

    let _guard = {
        let now = SystemTime::now();
        let seconds = now.duration_since(UNIX_EPOCH).unwrap().as_secs();
        timer.schedule(
            DateTime::from_timestamp(seconds.try_into().unwrap(), 0).unwrap(),
            Some(chrono::Duration::hours(24)),
            move || {
                println!("Creating and Publishing");
                create_and_publish();
            },
        )
    };
    thread::park();
    println!("end");
}

fn create_and_publish() {
    let config = get_config();

    let client = MyMastodonClient {
        base_url: config.base_url,
        access_token: config.access_token,
    };

    let provider = SentenceProvider::new(config.words_path);
    let message = provider.get_random_sentence();
    println!("created sentence: {:?}", message);
    client.publish_status(message)
}

fn get_config() -> Config {
    let file_path = "config.json";
    println!("Reading config from: {}", file_path);

    let contents = fs::read_to_string(file_path);

    let config = match contents {
        Ok(content) => Config::from_json(&content),
        Err(_) => panic!(),
    };

    println!("Read config:\n {:?}", config);
    return config;
}
