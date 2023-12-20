use tokio::runtime;

pub struct MyMastodonClient {
    pub base_url: String,
    pub access_token: String,
}

impl MyMastodonClient {
    pub fn publish_status(&self, message: String) {
        let client = megalodon::generator(
            megalodon::SNS::Mastodon,
            String::from(&self.base_url),
            Some(String::from(&self.access_token)),
            None,
        );
        let rt = runtime::Runtime::new().unwrap();
        let options = None;
        let res = rt.block_on(async { client.post_status(message, options).await });
        println!("{:?}", res);
    }
}
