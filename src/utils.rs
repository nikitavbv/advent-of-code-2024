use std::{env::var, fs::{read_to_string, write, exists}};

pub fn download_input(day: u32) -> String {
    let cached_path = format!("data/day_{}_input.txt", day);
    if exists(&cached_path).unwrap() {
        return read_to_string(cached_path).unwrap();
    }

    println!("downloading data from advent of code website");
    let client = reqwest::blocking::Client::new();
    let data = client.get(format!("https://adventofcode.com/2024/day/{}/input", day))
        .header("cookie", format!("session={}", var("SESSION_COOKIE").unwrap()))
        .send()
        .unwrap()
        .text()
        .unwrap();

    write(cached_path, data.clone()).unwrap();

    data
}
