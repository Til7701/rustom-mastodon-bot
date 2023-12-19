use std::fs;
use tokio::runtime;

mod config;
use crate::config::Config;

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
    let options = None;

    let client = megalodon::generator(
        megalodon::SNS::Mastodon,
        String::from(config.base_url),
        Some(String::from(config.access_token)),
        None,
    );
    let rt = runtime::Runtime::new().unwrap();
    let res = rt.block_on(async { client.post_status(message, options).await });
    println!("{:?}", res);
}
