use crate::utils::download_input;

pub mod part1;
pub mod part2;

pub fn parse_input() -> Vec<(u64, u64)> {
    download_input(1)
        .lines()
        .map(|v| {
            let spl = v.split(" ").filter(|v| !v.is_empty()).collect::<Vec<_>>();
            (spl[0].parse::<u64>().unwrap(), spl[1].parse::<u64>().unwrap())
        })
        .collect::<Vec<_>>()
}
