use std::fs;

mod config;
use crate::config::Config;
mod mastodon;
use crate::mastodon::MyMastodonClient;
mod sentence;
use crate::sentence::SentenceProvider;

fn main() {
    let config = get_config();

    //let client = MyMastodonClient {
    //    base_url: config.base_url,
    //    access_token: config.access_token,
    //};

    let provider = SentenceProvider {
        words_path: config.words_path,
    };
    let message = provider.get_random_sentence();
    println!("created sentence: {:?}", message);
    //client.publish_status(message)
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
