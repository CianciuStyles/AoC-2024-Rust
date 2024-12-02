fn parse_input(input: String) -> Vec<Vec<i32>> {
    input
        .split("\n")
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn check_report(report: &Vec<i32>) -> bool {
    let mut all_positives = true;
    let mut all_negatives = true;
    let mut out_of_range = false;

    report
        .iter()
        .zip(report.iter().skip(1))
        .for_each(|(level1, level2)| {
            let diff = level2 - level1;
            if diff < -3 || diff == 0 || diff > 3 {
                out_of_range = true
            }
            if diff > 0 {
                all_negatives = false
            } else {
                all_positives = false
            }
        });

    !out_of_range && (all_positives || all_negatives)
}

fn part1(input: &Vec<Vec<i32>>) -> usize {
    input
        .into_iter()
        .filter(|report| check_report(*report))
        .count()
}

fn part2(input: &Vec<Vec<i32>>) -> usize {
    input
        .into_iter()
        .filter(|report| {
            if check_report(*report) {
                return true;
            } else {
                for i in 0..report.len() {
                    let mut clone = (*report).clone();
                    clone.remove(i);
                    if check_report(&clone) {
                        return true;
                    }
                }
            }
            false
        })
        .count()
}

fn main() {
    let sample_input = parse_input(helpers::sample_file!());
    let sample_answer1 = part1(&sample_input);
    assert_eq!(sample_answer1, 2);
    let sample_answer2 = part2(&sample_input);
    assert_eq!(sample_answer2, 4);

    let input = parse_input(helpers::input_file!());
    let answer1 = part1(&input);
    println!("{}", answer1);
    let answer2 = part2(&input);
    println!("{}", answer2);
}
