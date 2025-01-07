use std::collections::{HashMap, HashSet};

type Input = (HashSet<String>, Vec<String>);

fn parse_input(text: String) -> Input {
    let parts: Vec<&str> = text.split("\n\n").collect();
    let patterns = parts[0].split(", ").map(ToString::to_string).collect();
    let desired_designs = parts[1].lines().map(ToString::to_string).collect();

    (patterns, desired_designs)
}

fn count_ways<'a>(
    design: &'a str,
    patterns: &HashSet<String>,
    memo: &mut HashMap<&'a str, u64>,
) -> u64 {
    if memo.contains_key(design) {
        return memo[design];
    }

    if design.is_empty() {
        return 1;
    }

    let mut num_ways = 0;
    for pattern in patterns {
        if design.starts_with(pattern) {
            num_ways += count_ways(&design[pattern.len()..], patterns, memo);
        }
    }

    memo.insert(design, num_ways);
    num_ways
}

fn part1((patterns, desired_designs): &Input) -> u64 {
    u64::try_from(
        desired_designs
            .iter()
            .filter(|design| count_ways(design, patterns, &mut HashMap::new()) > 0)
            .count(),
    )
    .expect("Invalid count")
}

fn part2((patterns, desired_designs): &Input) -> u64 {
    desired_designs
        .iter()
        .map(|design| count_ways(design, patterns, &mut HashMap::new()))
        .sum()
}

fn main() {
    let sample_text = helpers::sample_file!();
    let sample_input = parse_input(sample_text);
    let sample_answer1 = part1(&sample_input);
    assert_eq!(sample_answer1, 6);
    let sample_answer2 = part2(&sample_input);
    assert_eq!(sample_answer2, 16);

    let text = helpers::input_file!();
    let input = parse_input(text);
    let answer1 = part1(&input);
    println!("{answer1}");
    let answer2 = part2(&input);
    println!("{answer2}");
}
