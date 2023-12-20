use rand::seq::SliceRandom;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fs;

pub struct SentenceProvider {}

impl SentenceProvider {
    pub fn get_random_sentence() -> String {
        let file_path = "words.json";
        println!("Reading word list from: {}", file_path);
        let contents = fs::read_to_string(file_path);
        let word_list = match contents {
            Ok(content) => WordList::from_json(&content),
            Err(_) => panic!(),
        };

        let mut message: String = String::from("");

        let optional_word = Self::random_entry(&word_list.adjectives);
        message.push_str(&optional_word.expect(""));
        message.push_str(" ");

        let optional_word = Self::random_entry(&word_list.animals_and_plants);
        message.push_str(&optional_word.expect(""));
        message.push_str(" ");

        let optional_word = Self::random_entry(&word_list.verbs);
        message.push_str(&optional_word.expect(""));

        return message;
    }

    fn random_entry(v: &Vec<String>) -> Option<String> {
        v.choose(&mut rand::thread_rng()).cloned()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WordList {
    #[serde(rename = "Adjectives")]
    adjectives: Vec<String>,
    #[serde(rename = "Animals_and_Plants")]
    animals_and_plants: Vec<String>,
    #[serde(rename = "Body")]
    body: Vec<String>,
    #[serde(rename = "Clothing")]
    clothing: Vec<String>,
    #[serde(rename = "Colours")]
    colours: Vec<String>,
    #[serde(rename = "Family")]
    family: Vec<String>,
    #[serde(rename = "Food")]
    food: Vec<String>,
    #[serde(rename = "House")]
    house: Vec<String>,
    #[serde(rename = "Roles")]
    roles: Vec<String>,
    #[serde(rename = "School")]
    school: Vec<String>,
    #[serde(rename = "Story")]
    story: Vec<String>,
    #[serde(rename = "Verbs")]
    verbs: Vec<String>,
    #[serde(rename = "Other")]
    other: Vec<String>,
}

impl WordList {
    fn from_json(json: &String) -> WordList {
        serde_json::from_str(json).expect("JSON was not well-formatted")
    }
}
