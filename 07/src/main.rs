type Input = Vec<(u64, Vec<u64>)>;
fn parse_input(text: String) -> Input {
    text.lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            let test_value = parts[0].parse::<u64>().unwrap();
            let numbers: Vec<u64> = parts[1]
                .split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect();

            (test_value, numbers)
        })
        .collect()
}

fn check_calibration_recursive(
    target_value: u64,
    numbers: &[u64],
    operators: &Vec<&str>,
    current_index: usize,
    result_so_far: u64,
) -> bool {
    if result_so_far > target_value {
        return false;
    }
    if current_index == numbers.len() {
        return result_so_far == target_value;
    }

    let current_value = numbers[current_index];
    operators.iter().any(|op| {
        check_calibration_recursive(
            target_value,
            numbers,
            operators,
            current_index + 1,
            match *op {
                "+" => result_so_far + current_value,
                "*" => result_so_far * current_value,
                "||" => format!("{result_so_far}{current_value}")
                    .parse::<u64>()
                    .unwrap(),
                _ => result_so_far,
            },
        )
    })
}

fn check_calibration(target_value: u64, numbers: &[u64], operators: Vec<&str>) -> bool {
    check_calibration_recursive(target_value, &numbers[1..], &operators, 0, numbers[0])
}
fn part1(input: &Input) -> u64 {
    input
        .iter()
        .filter(|(test_value, numbers)| check_calibration(*test_value, numbers, vec!["+", "*"]))
        .map(|input| input.0)
        .sum()
}

fn part2(input: &Input) -> u64 {
    input
        .iter()
        .filter(|(test_value, numbers)| {
            check_calibration(*test_value, numbers, vec!["+", "*", "||"])
        })
        .map(|input| input.0)
        .sum()
}

fn main() {
    let sample_input = parse_input(helpers::sample_file!());
    let sample_answer1 = part1(&sample_input);
    assert_eq!(sample_answer1, 3749);
    let sample_answer2 = part2(&sample_input);
    assert_eq!(sample_answer2, 11387);

    let input = parse_input(helpers::input_file!());
    let answer1 = part1(&input);
    println!("{answer1}");
    let answer2 = part2(&input);
    println!("{answer2}");
}
