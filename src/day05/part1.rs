use {
    std::collections::HashMap,
    crate::utils::download_input,
};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct PageNumber(u32);

impl PageNumber {
    pub const fn new(number: u32) -> Self {
        Self(number)
    }
}

#[derive(Clone)]
struct Rule {
    page1: PageNumber,
    page2: PageNumber,
}

impl Rule {
    pub const fn new(page1: PageNumber, page2: PageNumber) -> Self {
        Self {
            page1,
            page2,
        }
    }
}

#[derive(Clone)]
struct Update {
    index: HashMap<PageNumber, usize>,
    middle_page_number: PageNumber,
}

impl Update {
    pub fn new(pages: Vec<PageNumber>) -> Self {
        let middle_page_number = pages.get(pages.len() / 2).unwrap().clone();
        let index = pages
            .into_iter()
            .enumerate()
            .map(|(index, page)| (page, index))
            .collect();

        Self {
            index,
            middle_page_number,
        }
    }

    pub fn satisfies_rule(&self, rule: &Rule) -> bool {
        let index1 = match self.index.get(&rule.page1) {
            Some(v) => v,
            None => return true,
        };
        let index2 = match self.index.get(&rule.page2) {
            Some(v) => v,
            None => return true,
        };
        index1 < index2
    }

    pub fn satisfies_rules(&self, rules: &[Rule]) -> bool {
        rules.iter().all(|rule| self.satisfies_rule(rule))
    }
}

#[allow(dead_code)]
pub fn run() {
    let input = download_input(5);
    let input = input.split("\n\n").collect::<Vec<_>>();

    let rules = input.get(0).unwrap()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let spl = line.split("|").collect::<Vec<_>>();
            Rule::new(
                PageNumber::new(spl.get(0).unwrap().parse().unwrap()),
                PageNumber::new(spl.get(1).unwrap().parse().unwrap()),
            )
        })
        .collect::<Vec<_>>();
    let updates = input.get(1).unwrap()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Update::new(line.split(",").map(|v| PageNumber::new(v.parse().unwrap())).collect()))
        .collect();

    println!("result: {}", solve(rules, updates));
}

fn solve(rules: Vec<Rule>, updates: Vec<Update>) -> u32 {
    updates
        .into_iter()
        .filter(|update| update.satisfies_rules(&rules))
        .map(|update| update.middle_page_number.0)
        .sum()
}

#[cfg(test)]
mod tests {
    use {
        lazy_static::lazy_static,
        super::*,
    };

    const EXAMPLE_RULES: [Rule; 21] = [
        Rule::new(PageNumber::new(47), PageNumber::new(53)),
        Rule::new(PageNumber::new(97), PageNumber::new(13)),
        Rule::new(PageNumber::new(97), PageNumber::new(61)),
        Rule::new(PageNumber::new(97), PageNumber::new(47)),
        Rule::new(PageNumber::new(75), PageNumber::new(29)),
        Rule::new(PageNumber::new(61), PageNumber::new(13)),
        Rule::new(PageNumber::new(75), PageNumber::new(53)),
        Rule::new(PageNumber::new(29), PageNumber::new(13)),
        Rule::new(PageNumber::new(97), PageNumber::new(29)),
        Rule::new(PageNumber::new(53), PageNumber::new(29)),
        Rule::new(PageNumber::new(61), PageNumber::new(53)),
        Rule::new(PageNumber::new(97), PageNumber::new(53)),
        Rule::new(PageNumber::new(61), PageNumber::new(29)),
        Rule::new(PageNumber::new(47), PageNumber::new(13)),
        Rule::new(PageNumber::new(75), PageNumber::new(47)),
        Rule::new(PageNumber::new(97), PageNumber::new(75)),
        Rule::new(PageNumber::new(47), PageNumber::new(61)),
        Rule::new(PageNumber::new(75), PageNumber::new(61)),
        Rule::new(PageNumber::new(47), PageNumber::new(29)),
        Rule::new(PageNumber::new(75), PageNumber::new(13)),
        Rule::new(PageNumber::new(53), PageNumber::new(13)),
    ];

    lazy_static! {
        static ref EXAMPLE_UPDATE_1: Update = Update::new(vec![
            PageNumber::new(75),
            PageNumber::new(47),
            PageNumber::new(61),
            PageNumber::new(53),
            PageNumber::new(29),
        ]);

        static ref EXAMPLE_UPDATE_2: Update = Update::new(vec![
            PageNumber::new(97),
            PageNumber::new(61),
            PageNumber::new(53),
            PageNumber::new(29),
            PageNumber::new(13),
        ]);

        static ref EXAMPLE_UPDATE_3: Update = Update::new(vec![
            PageNumber::new(75),
            PageNumber::new(29),
            PageNumber::new(13),
        ]);

        static ref EXAMPLE_UPDATE_4: Update = Update::new(vec![
            PageNumber::new(75),
            PageNumber::new(97),
            PageNumber::new(47),
            PageNumber::new(61),
            PageNumber::new(53),
        ]);

        static ref EXAMPLE_UPDATE_5: Update = Update::new(vec![
            PageNumber::new(61),
            PageNumber::new(13),
            PageNumber::new(29),
        ]);

        static ref EXAMPLE_UPDATE_6: Update = Update::new(vec![
            PageNumber::new(97),
            PageNumber::new(13),
            PageNumber::new(75),
            PageNumber::new(29),
            PageNumber::new(47),
        ]);
    }

    #[test]
    fn test_example_update_1() {
        assert!(EXAMPLE_UPDATE_1.satisfies_rules(&EXAMPLE_RULES));
        assert_eq!(EXAMPLE_UPDATE_1.middle_page_number, PageNumber::new(61));
    }

    #[test]
    fn test_example_update_2() {
        assert!(EXAMPLE_UPDATE_2.satisfies_rules(&EXAMPLE_RULES));
        assert_eq!(EXAMPLE_UPDATE_2.middle_page_number, PageNumber::new(53));
    }

    #[test]
    fn test_example_update_3() {
        assert!(EXAMPLE_UPDATE_3.satisfies_rules(&EXAMPLE_RULES));
        assert_eq!(EXAMPLE_UPDATE_3.middle_page_number, PageNumber::new(29));
    }

    #[test]
    fn test_example_update_4() {
        assert!(!EXAMPLE_UPDATE_4.satisfies_rules(&EXAMPLE_RULES));
    }

    #[test]
    fn test_example_update_5() {
        assert!(!EXAMPLE_UPDATE_5.satisfies_rules(&EXAMPLE_RULES));
    }

    #[test]
    fn test_example_update_6() {
        assert!(!&EXAMPLE_UPDATE_6.satisfies_rules(&EXAMPLE_RULES));
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
            143
        );
    }
}
