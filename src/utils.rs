use std::{env::var, fs::{read_to_string, write, exists}};

pub fn download_input() -> String {
    if exists("data/day_1_input.txt").unwrap() {
        return read_to_string("data/day_1_input.txt").unwrap();
    }

    println!("downloading data from advent of code website");
    let client = reqwest::blocking::Client::new();
    let data = client.get("https://adventofcode.com/2024/day/1/input")
        .header("cookie", format!("session={}", var("SESSION_COOKIE").unwrap()))
        .send()
        .unwrap()
        .text()
        .unwrap();

    write("data/day_1_input.txt", data.clone()).unwrap();

    data
}
