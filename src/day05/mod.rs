use {
    std::collections::HashMap,
    lazy_static::lazy_static,
    crate::utils::download_input,
};

#[cfg(test)]
pub const EXAMPLE_RULES: [Rule; 21] = [
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
    pub static ref EXAMPLE_UPDATE_1: Update = Update::new(vec![
        PageNumber::new(75),
        PageNumber::new(47),
        PageNumber::new(61),
        PageNumber::new(53),
        PageNumber::new(29),
    ]);

    pub static ref EXAMPLE_UPDATE_2: Update = Update::new(vec![
        PageNumber::new(97),
        PageNumber::new(61),
        PageNumber::new(53),
        PageNumber::new(29),
        PageNumber::new(13),
    ]);

    pub static ref EXAMPLE_UPDATE_3: Update = Update::new(vec![
        PageNumber::new(75),
        PageNumber::new(29),
        PageNumber::new(13),
    ]);

    pub static ref EXAMPLE_UPDATE_4: Update = Update::new(vec![
        PageNumber::new(75),
        PageNumber::new(97),
        PageNumber::new(47),
        PageNumber::new(61),
        PageNumber::new(53),
    ]);

    pub static ref EXAMPLE_UPDATE_5: Update = Update::new(vec![
        PageNumber::new(61),
        PageNumber::new(13),
        PageNumber::new(29),
    ]);

    pub static ref EXAMPLE_UPDATE_6: Update = Update::new(vec![
        PageNumber::new(97),
        PageNumber::new(13),
        PageNumber::new(75),
        PageNumber::new(29),
        PageNumber::new(47),
    ]);
}

pub mod part1;
pub mod part2;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct PageNumber(u32);

impl PageNumber {
    pub const fn new(number: u32) -> Self {
        Self(number)
    }
}

#[derive(Clone)]
pub struct Rule {
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
pub struct Update {
    index: HashMap<PageNumber, usize>,
    pages: Vec<PageNumber>,
}

impl Update {
    pub fn empty() -> Self {
        Self {
            index: HashMap::new(),
            pages: Vec::new(),
        }
    }

    pub fn new(pages: Vec<PageNumber>) -> Self {
        let index = pages
            .iter()
            .enumerate()
            .map(|(index, page)| (page.clone(), index))
            .collect();

        Self {
            index,
            pages,
        }
    }

    pub fn append(&self, page: PageNumber) -> Self {
        let mut index = self.index.clone();
        index.insert(page.clone(), index.len());

        let mut pages = self.pages.clone();
        pages.push(page);

        Self {
            index,
            pages,
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

    pub fn middle_page_number(&self) -> &PageNumber {
        self.pages.get(self.pages.len() / 2).unwrap()
    }
}

impl std::fmt::Debug for Update {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.pages.fmt(f)
    }
}

impl PartialEq for Update {
    fn eq(&self, other: &Self) -> bool {
        self.pages.eq(&other.pages)
    }

    fn ne(&self, other: &Self) -> bool {
        self.pages.ne(&other.pages)
    }
}

pub fn parse_input() -> (Vec<Rule>, Vec<Update>) {
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

    (rules, updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_update_1() {
        assert!(EXAMPLE_UPDATE_1.satisfies_rules(&EXAMPLE_RULES));
        assert_eq!(EXAMPLE_UPDATE_1.middle_page_number(), &PageNumber::new(61));
    }

    #[test]
    fn test_example_update_2() {
        assert!(EXAMPLE_UPDATE_2.satisfies_rules(&EXAMPLE_RULES));
        assert_eq!(EXAMPLE_UPDATE_2.middle_page_number(), &PageNumber::new(53));
    }

    #[test]
    fn test_example_update_3() {
        assert!(EXAMPLE_UPDATE_3.satisfies_rules(&EXAMPLE_RULES));
        assert_eq!(EXAMPLE_UPDATE_3.middle_page_number(), &PageNumber::new(29));
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
}
