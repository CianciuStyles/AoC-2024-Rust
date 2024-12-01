use std::collections::HashMap;
use std::iter::zip;

use helpers;

type Input = (Vec<u32>, Vec<u32>);

fn part1((left_list, right_list): &Input) -> u32 {
    let mut sorted_left_list = left_list.clone();
    sorted_left_list.sort_unstable();

    let mut sorted_right_list = right_list.clone();
    sorted_right_list.sort_unstable();

    zip(sorted_left_list, sorted_right_list)
        .map(|(left, right)| left.abs_diff(right))
        .sum()
}

fn part2((left_list, right_list): &Input) -> u32 {
    let mut counter = HashMap::<u32, u32>::new();
    for item in right_list {
        *counter.entry(*item).or_insert(0) += 1;
    }

    left_list
        .iter()
        .map(|value| value * *counter.entry(*value).or_default())
        .sum()
}

fn parse_input(input: String) -> Input {
    let (mut left_list, mut right_list) = (Vec::new(), Vec::new());

    for line in input.lines() {
        let values: Vec<&str> = line.split_whitespace().collect();
        left_list.push(values[0].parse::<u32>().unwrap());
        right_list.push(values[1].parse::<u32>().unwrap());
    }

    (left_list, right_list)
}

fn main() {
    let sample_input = parse_input(helpers::sample_file!());
    let sample_answer1 = part1(&sample_input);
    assert_eq!(11, sample_answer1);
    let sample_answer2 = part2(&sample_input);
    assert_eq!(31, sample_answer2);

    let input = parse_input(helpers::input_file!());
    let answer1 = part1(&input);
    println!("{}", answer1);
    let answer2 = part2(&input);
    println!("{}", answer2);
}
