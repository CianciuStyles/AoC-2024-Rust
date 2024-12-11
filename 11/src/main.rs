use std::collections::HashMap;

fn parse_input(text: String) -> Vec<u64> {
    text.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn calculate_stones(input: &Vec<u64>, num_rounds: usize) -> u64 {
    let mut current_counter = HashMap::new();
    for number in input {
        *current_counter.entry(*number).or_insert(0) += 1;
    }

    (0..num_rounds).for_each(|_| {
        let mut next_counter = HashMap::new();
        for (number, count) in &current_counter {
            if *number == 0 {
                *next_counter.entry(1).or_insert(0) += count;
                continue;
            }

            let num_string = number.to_string();
            let num_digits = num_string.len();

            if num_digits % 2 == 0 {
                let mid = num_digits / 2;
                let value1 = num_string[..mid].parse().unwrap();
                let value2 = num_string[mid..].parse().unwrap();
                *next_counter.entry(value1).or_insert(0) += count;
                *next_counter.entry(value2).or_insert(0) += count;
            } else {
                *next_counter.entry(number * 2024).or_insert(0) += count;
            }
        }

        current_counter = next_counter;
    });

    current_counter.values().sum()
}

fn part1(input: &Vec<u64>) -> u64 {
    calculate_stones(input, 25)
}

fn part2(input: &Vec<u64>) -> u64 {
    calculate_stones(input, 75)
}

fn main() {
    let sample_text = helpers::sample_file!();
    let sample_input = parse_input(sample_text);
    let sample_answer1 = part1(&sample_input);
    assert_eq!(sample_answer1, 55312);

    let text = helpers::input_file!();
    let input = parse_input(text);
    let answer1 = part1(&input);
    println!("{answer1}");
    let answer2 = part2(&input);
    println!("{answer2}");
}
