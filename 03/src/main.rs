use regex::Regex;

fn part1(input: &String) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|captures| {
            let num1 = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let num2 = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
            num1 * num2
        })
        .sum()
}

fn part2(input: &String) -> i32 {
    let mut enabled = true;
    let mut result = 0;

    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    re.captures_iter(input)
        .for_each(|captures| match captures.get(0).unwrap().as_str() {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                if enabled {
                    let num1 = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
                    let num2 = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
                    result += num1 * num2
                }
            }
        });

    result
}

fn main() {
    let sample_input = helpers::sample_file!();
    let sample_answer1 = part1(&sample_input);
    assert_eq!(sample_answer1, 161);
    let sample_answer2 = part2(&sample_input);
    assert_eq!(sample_answer2, 48);

    let input = helpers::input_file!();
    let answer1 = part1(&input);
    println!("{}", answer1);
    let answer2 = part2(&input);
    println!("{}", answer2);
}
