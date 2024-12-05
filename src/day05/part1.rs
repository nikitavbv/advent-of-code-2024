use super::{
    Rule,
    Update,
    parse_input,
};

#[allow(dead_code)]
pub fn run() {
    let (rules, updates) = parse_input();
    println!("result: {}", solve(rules, updates));
}

fn solve(rules: Vec<Rule>, updates: Vec<Update>) -> u32 {
    updates
        .into_iter()
        .filter(|update| update.satisfies_rules(&rules))
        .map(|update| update.middle_page_number().0)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{
        *,
        super::{
            EXAMPLE_RULES,
            EXAMPLE_UPDATE_1,
            EXAMPLE_UPDATE_2,
            EXAMPLE_UPDATE_3,
            EXAMPLE_UPDATE_4,
            EXAMPLE_UPDATE_5,
            EXAMPLE_UPDATE_6,
        },
    };

    #[test]
    fn test_solve_example() {
        assert_eq!(
            solve(
                EXAMPLE_RULES.to_vec(),
                vec![
                    EXAMPLE_UPDATE_1.clone(),
                    EXAMPLE_UPDATE_2.clone(),
                    EXAMPLE_UPDATE_3.clone(),
                    EXAMPLE_UPDATE_4.clone(),
                    EXAMPLE_UPDATE_5.clone(),
                    EXAMPLE_UPDATE_6.clone(),
                ],
            ),
            143
        );
    }
}
