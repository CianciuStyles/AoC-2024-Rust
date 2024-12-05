use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

type Input1 = HashMap<u32, HashSet<u32>>;
type Input2 = Vec<Vec<u32>>;
type Input = (Input1, Input2);

fn parse_page_ordering_rules(text: &str) -> Input1 {
    let mut rules = HashMap::new();
    
    for rule in text.lines() {
        let parts: Vec<&str> = rule.split('|').collect();
        let before = parts[0].parse::<u32>().unwrap();
        let after = parts[1].parse::<u32>().unwrap();
    
        rules
            .entry(before)
            .or_insert_with(HashSet::new)
            .insert(after);
    }
    
    rules
}

fn parse_updates(text: &str) -> Vec<Vec<u32>> {
    text.split('\n')
        .map(|line| {
            line.split(',')
                .map(|number| number.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}
fn parse_input(text: &str) -> Input {
    let parts: Vec<&str> = text.split("\n\n").collect();
    (parse_page_ordering_rules(parts[0]), parse_updates(parts[1]))
}

fn get_ordering_rules(page_ordering_rules: &Input1, page: u32) -> HashSet<u32> {
    match page_ordering_rules.get(&page) {
        Some(rules) => rules.clone(),
        None => HashSet::new(),
    }
}

fn is_valid(update: &Vec<u32>, page_ordering_rules: &Input1) -> bool {
    let mut following_pages: HashSet<u32> = update.iter().copied().collect::<HashSet<u32>>();
    for page in update {
        following_pages.remove(page);
        if following_pages
            .difference(&get_ordering_rules(page_ordering_rules, *page))
            .count()
            > 0
        {
            return false;
        }
    }
    true
}

fn take_middle_value(vector: &[u32]) -> u32 {
    vector[vector.len() / 2]
}

fn part1((page_ordering_rules, updates): &Input) -> u32 {
    updates
        .iter()
        .filter(|update| is_valid(update, page_ordering_rules))
        .map(|update| take_middle_value(update))
        .sum()
}

fn correct_update(update: &[u32], page_ordering_rules: &Input1) -> Vec<u32> {
    let mut new_update: Vec<u32> = update.to_vec().clone();
    new_update.sort_by(|page1, page2| {
        if get_ordering_rules(page_ordering_rules, *page1).contains(page2) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    new_update
}

fn part2((page_ordering_rules, updates): &Input) -> u32 {
    updates
        .iter()
        .filter(|update| !is_valid(update, page_ordering_rules))
        .map(|update| correct_update(update, page_ordering_rules))
        .map(|update| take_middle_value(&update))
        .sum()
}

fn main() {
    let sample_text = helpers::sample_file!();
    let sample_input = parse_input(&sample_text);
    let sample_answer1 = part1(&sample_input);
    assert_eq!(sample_answer1, 143);
    let sample_answer2 = part2(&sample_input);
    assert_eq!(sample_answer2, 123);

    let text = helpers::input_file!();
    let input = parse_input(&text);
    let answer1 = part1(&input);
    println!("{answer1}");
    let answer2 = part2(&input);
    println!("{answer2}");
}
