extern crate rand;
use rand::seq::IteratorRandom;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct SentenceProvider {
    pub words_path: String,
}

impl SentenceProvider {
    pub fn get_random_sentence(&self) -> String {
        let sentence_pattern = self.random_pattern();

        return self.parse_pattern(&sentence_pattern);
    }

    fn random_pattern(&self) -> String {
        let file_path = "sentence_patterns";
        return SentenceProvider::get_random_line_of_file(file_path);
    }

    fn parse_pattern(&self, pattern: &str) -> String {
        let mut sentence: String = String::from("");

        for i in 0..=pattern.len() - 1 {
            let c = pattern.chars().nth(i).unwrap().to_string();
            sentence.push_str(&c);
        }

        return sentence;
    }

    fn get_random_line_of_file(path: &str) -> String {
        let f =
            File::open(&path).unwrap_or_else(|e| panic!("(;_;) file not found: {}: {}", &path, e));
        let r = BufReader::new(f);
        let res = r
            .lines()
            .choose(&mut rand::thread_rng())
            .expect("File had no lines");
        match res {
            Ok(line) => return line,
            Err(_) => panic!(),
        }
    }
}
