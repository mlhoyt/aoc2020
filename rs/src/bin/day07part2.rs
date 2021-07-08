use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input/day07.txt");

    let rules = input
        .lines()
        .map(parse_rule)
        .filter(|v| !v.is_none())
        .map(|v| v.unwrap())
        .collect::<Rules>();

    let result = count_contained_bags(&rules, &Bag::new("shiny", "gold"));
    println!("{}", result);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Bag {
    adjective: String,
    color: String,
}

impl Bag {
    fn new(adjective: &str, color: &str) -> Self {
        Bag {
            adjective: adjective.to_string(),
            color: color.to_string(),
        }
    }
}

#[derive(Debug)]
struct CollectedBag {
    count: u32,
    bag: Bag,
}

impl CollectedBag {
    fn new(count: u32, bag: Bag) -> Self {
        CollectedBag { count, bag }
    }
}

type Rules = HashMap<Bag, Vec<CollectedBag>>;

fn count_contained_bags(rules: &Rules, bag: &Bag) -> u32 {
    let mut result = 0;
    rules.get(&bag).unwrap().iter().for_each(|v| {
        result += v.count + (v.count * count_contained_bags(rules, &v.bag));
    });

    result
}

fn parse_rule(line: &str) -> Option<(Bag, Vec<CollectedBag>)> {
    let re_bag = Regex::new(r"^(\S+)\s+(\S+)\s+bags\s+contain(.*)$").unwrap();
    let re_no_collected_bags = Regex::new(r"\s*no\s+other\s+bags").unwrap();
    let re_collected_bag = Regex::new(r"\s*,?\s*(\d+)\s+(\S+)\s+(\S+)\s+bag[s]?(.*)$").unwrap();

    match re_bag.is_match(line) {
        true => {
            let elems = re_bag.captures(line).unwrap();

            let adjective = elems.get(1).map(|m| m.as_str()).unwrap();
            let color = elems.get(2).map(|m| m.as_str()).unwrap();
            let bag = Bag::new(adjective, color);

            let mut collected_bags = Vec::<CollectedBag>::new();
            let mut line = elems.get(3).map(|m| m.as_str()).unwrap();
            if re_no_collected_bags.is_match(line) {
                return Some((bag, collected_bags));
            }
            while re_collected_bag.is_match(line) {
                let elems = re_collected_bag.captures(line).unwrap();
                let count = elems
                    .get(1)
                    .map(|m| m.as_str())
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();

                let adjective = elems.get(2).map(|m| m.as_str()).unwrap();
                let color = elems.get(3).map(|m| m.as_str()).unwrap();
                let bag = Bag::new(adjective, color);
                collected_bags.push(CollectedBag::new(count, bag));

                line = elems.get(4).map(|m| m.as_str()).unwrap();
            }
            return Some((bag, collected_bags));
        }
        false => None,
    }
}
