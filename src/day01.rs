use std::{env::var, fs::{read_to_string, write, exists}};

pub fn run() {
    let input = download_input();
    let parsed = input.lines()
        .map(|v| {
            let spl = v.split(" ").filter(|v| !v.is_empty()).collect::<Vec<_>>();
            (spl[0].parse::<u64>().unwrap(), spl[1].parse::<u64>().unwrap())
        })
        .collect::<Vec<_>>();

    println!("result is: {}", solve(parsed));
}

fn download_input() -> String {
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

fn solve(input: Vec<(u64, u64)>) -> u64 {
    let mut first = input.iter().map(|v| v.0).collect::<Vec<_>>();
    let mut second = input.iter().map(|v| v.1).collect::<Vec<_>>();

    first.sort();
    second.sort();

    first.into_iter().zip(second.into_iter()).map(|v| ((v.0 as i64) - (v.1 as i64)).abs() as u64).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(solve(vec![
            (3, 4),
            (4, 3),
            (2, 5),
            (1, 3),
            (3, 9),
            (3, 3),
        ]), 11);
    }
}
