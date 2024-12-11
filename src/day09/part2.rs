use {
    crate::utils::download_input,
    super::parse_disk_map,
};

#[allow(dead_code)]
pub fn run() {
    println!("result: {}", solve(&download_input(9)));
}

fn solve(disk_map: &str) -> u64 {
    let mut disk_map = parse_disk_map(disk_map);
    disk_map.defragment_contiguous_files();
    disk_map.checksum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(solve("2333133121414131402"), 2858);
    }
}
