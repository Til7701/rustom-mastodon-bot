extern crate rand;
use rand::seq::IteratorRandom;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_random_line_of_file(path: &str) -> String {
    let f = File::open(&path).unwrap_or_else(|e| panic!("(;_;) file not found: {}: {}", &path, e));
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
