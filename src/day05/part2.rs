use {
    std::cmp::Ordering,
    super::{PageNumber, Update, Rule, parse_input},
};

#[allow(dead_code)]
pub fn run() {
    let (rules, updates) = parse_input();
    println!("result: {}", solve(rules, updates));
}

fn solve(rules: Vec<Rule>, updates: Vec<Update>) -> u32 {
    updates
        .into_iter()
        .filter(|update| !update.satisfies_rules(&rules))
        .map(|update| create_correctly_ordered(&rules, &update.pages))
        .map(|update| update.middle_page_number().0)
        .sum()
}

pub fn create_correctly_ordered(rules: &[Rule], pages: &[PageNumber]) -> Update {
    let mut pages = pages.to_vec();
    pages.sort_by(|a, b| {
        let rule1 = rules.iter().find(|rule| &rule.page1 == a && &rule.page2 == b);
        let rule2 = rules.iter().find(|rule| &rule.page1 == b && &rule.page2 == a);

        if rule1.is_some() {
            Ordering::Less
        } else if rule2.is_some() {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });

    Update::new(pages.to_vec())
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
    fn test_example_update_4_fixed() {
        assert_eq!(
            create_correctly_ordered(&EXAMPLE_RULES, &EXAMPLE_UPDATE_4.pages),
            Update::new(vec![
                PageNumber::new(97),
                PageNumber::new(75),
                PageNumber::new(47),
                PageNumber::new(61),
                PageNumber::new(53),
            ]),
        );
    }

    #[test]
    fn test_example_update_5_fixed() {
        assert_eq!(
            create_correctly_ordered(&EXAMPLE_RULES, &EXAMPLE_UPDATE_5.pages),
            Update::new(vec![
                PageNumber::new(61),
                PageNumber::new(29),
                PageNumber::new(13),
            ]),
        );
    }

    #[test]
    fn test_example_update_6_fixed() {
        assert_eq!(
            create_correctly_ordered(&EXAMPLE_RULES, &EXAMPLE_UPDATE_6.pages),
            Update::new(vec![
                PageNumber::new(97),
                PageNumber::new(75),
                PageNumber::new(47),
                PageNumber::new(29),
                PageNumber::new(13),
            ]),
        );
    }

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
            123
        );
    }
}
