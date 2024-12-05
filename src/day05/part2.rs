use {
    std::collections::HashSet,
    indicatif::ProgressIterator,
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
        .progress()
        .filter(|update| !update.satisfies_rules(&rules))
        .map(|update| create_correctly_ordered(&rules, &update.pages))
        .map(|update| update.middle_page_number().0)
        .sum()
}

pub fn create_correctly_ordered(rules: &[Rule], pages: &[PageNumber]) -> Update {
    let pages_set = pages.iter().cloned().collect::<HashSet<_>>();
    let rules_subset = rules
        .iter()
        .filter(|rule| pages_set.contains(&rule.page1) && pages_set.contains(&rule.page2))
        .cloned()
        .collect::<Vec<_>>();

    create_correctly_ordered_with_update(&rules_subset, pages, &Update::empty())
        .expect("no way to create correctly ordered update")
}

pub fn create_correctly_ordered_with_update(rules: &[Rule], pages: &[PageNumber], update: &Update) -> Option<Update> {
    if !update.satisfies_rules(&rules) {
        return None;
    }
    if pages.is_empty() {
        return Some(update.clone());
    }

    for i in 0..pages.len() {
        let page = pages[i].clone();
        let mut pages_without_page = pages[0..i].to_vec();
        pages_without_page.append(&mut pages[i+1..].to_vec());

        let updated_update = update.append(page);
        let result = create_correctly_ordered_with_update(rules, &pages_without_page, &updated_update);
        if let Some(result) = result {
            return Some(result);
        }
    }

    None
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
