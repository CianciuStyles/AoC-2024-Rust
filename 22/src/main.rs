use std::collections::HashMap;
type Sequence = (i64, i64, i64, i64);

fn parse_input(text: String) -> Vec<u32> {
    text.lines()
        .map(|x| x.parse().expect("Invalid value"))
        .collect()
}

fn generate_secret_number(seed: u32) -> i64 {
    let mut curr_number = i64::from(seed);

    for _ in 0..2000 {
        curr_number = ((curr_number * 64) ^ curr_number) % 16_777_216;
        curr_number = ((curr_number / 32) ^ curr_number) % 16_777_216;
        curr_number = ((curr_number * 2048) ^ curr_number) % 16_777_216;
    }

    curr_number
}

fn part1(input: &[u32]) -> i64 {
    input.iter().map(|seed| generate_secret_number(*seed)).sum()
}

fn generate_prices(seed: u32) -> Vec<i64> {
    let mut prices = Vec::new();
    let mut curr_number = i64::from(seed);

    for _ in 0..=2000 {
        prices.push(curr_number % 10);
        curr_number = ((curr_number * 64) ^ curr_number) % 16_777_216;
        curr_number = ((curr_number / 32) ^ curr_number) % 16_777_216;
        curr_number = ((curr_number * 2048) ^ curr_number) % 16_777_216;
    }

    prices
}

fn find_sequences(prices: &[i64]) -> HashMap<Sequence, i64> {
    let mut sequences = HashMap::new();

    for i in 0..prices.len() - 4 {
        sequences
            .entry((
                prices[i + 1] - prices[i],
                prices[i + 2] - prices[i + 1],
                prices[i + 3] - prices[i + 2],
                prices[i + 4] - prices[i + 3],
            ))
            .or_insert(prices[i + 4]);
    }

    sequences
}

fn part2(input: &[u32]) -> i64 {
    let sequences: Vec<HashMap<Sequence, i64>> = input
        .iter()
        .map(|seed| generate_prices(*seed))
        .map(|prices| find_sequences(&prices))
        .collect();

    sequences[0]
        .keys()
        .map(|candidate| {
            sequences
                .iter()
                .map(|sequences_map| sequences_map.get(candidate).unwrap_or(&0))
                .sum::<i64>()
        })
        .max()
        .expect("Invalid result")
}

fn main() {
    let sample_text = helpers::sample_file!();
    let sample_input = parse_input(sample_text);
    let sample_answer1 = part1(&sample_input);
    assert_eq!(sample_answer1, 37_327_623);

    let text = helpers::input_file!();
    let input = parse_input(text);
    let answer1 = part1(&input);
    println!("{answer1}");
    let answer2 = part2(&input);
    println!("{answer2}");
}
