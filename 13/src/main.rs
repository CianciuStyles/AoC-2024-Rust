use regex::Regex;

type Case = (i64, i64, i64, i64, i64, i64);
type Input = Vec<Case>;

fn extract_values(line: &str, regex: &Regex) -> (i64, i64) {
    regex
        .captures(line)
        .map(|captures| {
            (
                captures[1].parse::<i64>().unwrap(),
                captures[2].parse::<i64>().unwrap(),
            )
        })
        .unwrap()
}

fn parse_input(text: String) -> Input {
    let a_regex = Regex::new(r"^Button A: X\+(\d+), Y\+(\d+)$").unwrap();
    let b_regex = Regex::new(r"^Button B: X\+(\d+), Y\+(\d+)$").unwrap();
    let prize_regex = Regex::new(r"^Prize: X=(\d+), Y=(\d+)$").unwrap();

    text.split("\n\n")
        .map(|section| {
            let lines: Vec<&str> = section.lines().collect();

            let (a_x, a_y) = extract_values(lines[0], &a_regex);
            let (b_x, b_y) = extract_values(lines[1], &b_regex);
            let (prize_x, prize_y) = extract_values(lines[2], &prize_regex);

            (a_x, a_y, b_x, b_y, prize_x, prize_y)
        })
        .collect()
}

fn find_tokens(a_x: i64, a_y: i64, b_x: i64, b_y: i64, prize_x: i64, prize_y: i64) -> i64 {
    let a_denom = b_y * prize_x - b_x * prize_y;
    let a_num = a_x * b_y - a_y * b_x;
    if a_denom % a_num != 0 {
        return 0;
    }
    let a_presses = a_denom / a_num;
    
    let b_denom = prize_x - a_x * a_presses;
    if b_denom % b_x != 0 {
        return 0;
    }
    let b_presses = b_denom / b_x;
    
    3 * a_presses + b_presses
}

fn part1(input: &Input) -> i64 {
    input
        .iter()
        .map(|(a_x, a_y, b_x, b_y, prize_x, prize_y)| {
            find_tokens(*a_x, *a_y, *b_x, *b_y, *prize_x, *prize_y)
        })
        .sum()
}

fn part2(input: &Input) -> i64 {
    input
        .iter()
        .map(|(a_x, a_y, b_x, b_y, prize_x, prize_y)| {
            find_tokens(
                *a_x,
                *a_y,
                *b_x,
                *b_y,
                *prize_x + 10_000_000_000_000,
                *prize_y + 10_000_000_000_000,
            )
        })
        .sum()
}

fn main() {
    let sample_text = helpers::sample_file!();
    let sample_input = parse_input(sample_text);
    let sample_answer1 = part1(&sample_input);
    assert_eq!(sample_answer1, 480);

    let text = helpers::input_file!();
    let input = parse_input(text);
    let answer1 = part1(&input);
    println!("{answer1}");
    let answer2 = part2(&input);
    println!("{answer2}");
}
