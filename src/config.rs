use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub base_url: String,
    pub access_token: String,
    pub words_path: String,
}

impl Config {
    pub fn from_json(json: &String) -> Config {
        serde_json::from_str(json).expect("JSON was not well-formatted")
    }
}
