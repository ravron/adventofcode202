use std::collections::HashMap;
use regex::Regex;

pub fn day7() {
    let (p1, p2) = day7_impl();
    println!("part 1: {}", p1);
    println!("part 2: {}", p2);
}

struct BagRules<'a>(
    HashMap<&'a str, Vec<BagRule<'a>>>,
);

#[derive(Debug)]
struct BagRule<'a>{
    name: &'a str,
    count: usize,
}

fn day7_impl() -> (usize, usize) {
    const TARGET_BAG: &str = "shiny gold";

    let input = include_str!("../inputs/day7.txt");

    let rules = BagRules::from_input(input);

    (rules.count_can_contain(TARGET_BAG),
     rules.count_contains(TARGET_BAG))
}

impl<'a> BagRules<'a> {
    fn from_input(input: &'a str) -> Self {
        lazy_static!{
            static ref BAGS_RE: regex::Regex = Regex::new(r"(?:(\d+) )?(\w+ \w+) bag").unwrap();
        }

        let mut rules: HashMap<&str, Vec<BagRule>> = HashMap::new();
        for rule in input.lines() {
            let mut matches: regex::CaptureMatches = BAGS_RE.captures_iter(rule);
            let subject = matches.next().unwrap().get(2).unwrap().as_str();
            let mut objects: Vec<BagRule> = vec![];

            for capture in matches {
                match capture.get(2).unwrap().as_str() {
                    "no other" => break,
                    color => objects.push(BagRule{
                        name: color,
                        count: capture.get(1).unwrap().as_str().parse().unwrap(),
                    }),
                }
            }

            rules.insert(subject, objects);
        }
        BagRules(rules)
    }

    fn can_contain(&self, container: &str, containee: &str) -> bool {
        self.0[container].iter()
            .any(|br| br.name == containee) ||
            self.0[container].iter()
                .any(|br| self.can_contain(br.name, containee))
    }

    fn count_can_contain(&self, containee: &str) -> usize {
        self.0.keys()
            .filter(|k| self.can_contain(k, containee))
            .count()
    }

    fn count_contains(&self, container: &str) -> usize {
        self.0[container].iter()
            .map(|br| {
                br.count + (br.count * self.count_contains(br.name))
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_day7 () {
        let (p1, p2) = day7_impl();
        assert_eq!(p1, 268);
        assert_eq!(p2, 7867);
    }

    #[bench]
    fn bench_day7(b: &mut Bencher) {
        b.iter(day7_impl);
    }
}