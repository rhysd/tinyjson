use std::fs;
use std::path::PathBuf;

pub fn load_my_2020_dec_tweets() -> String {
    let mut p = PathBuf::from("assets");
    p.push("tweets_2020_dec.json");
    fs::read_to_string(p).unwrap()
}
