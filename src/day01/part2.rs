use {
    rpds::HashTrieMap,
    super::parse_input,
};

pub fn run() {
    println!("result is: {}", solve(parse_input()));
}

fn solve(input: Vec<(u64, u64)>) -> u64 {
    let first = input.iter().map(|v| v.0).collect::<Vec<_>>();
    let second = input.iter().map(|v| v.1).collect::<Vec<_>>();

    let table = first
        .into_iter()
        .fold(HashTrieMap::new(), |table, item| table.insert(item, table.get(&item).unwrap_or(&0) + 1));

    second.into_iter().map(|v| v * *(table.get(&v).unwrap_or(&0))).sum()
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
        ]), 31);
    }
}
