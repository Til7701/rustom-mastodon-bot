use std::fs;

mod config;
use crate::config::Config;
mod mastodon;
use crate::mastodon::MyMastodonClient;

fn main() {
    let file_path = "config.json";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path);

    let config = match contents {
        Ok(content) => Config::from_json(&content),
        Err(_) => panic!(),
    };

    println!("Read config:\n {:?}", config);

    let message = String::from("Test Rust");
    let client = MyMastodonClient {
        base_url: config.base_url,
        access_token: config.access_token,
    };
    client.publish_status(message);
}
