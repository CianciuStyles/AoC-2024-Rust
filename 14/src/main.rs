use regex::Regex;
use std::collections::HashSet;

type Position = (i32, i32);
type Velocity = (i32, i32);

fn parse_input(text: String) -> Vec<(Position, Velocity)> {
    let regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    text.lines()
        .map(|line| {
            regex
                .captures(line)
                .map(|captures| {
                    (
                        (
                            captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                            captures.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                        ),
                        (
                            captures.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                            captures.get(4).unwrap().as_str().parse::<i32>().unwrap(),
                        ),
                    )
                })
                .unwrap()
        })
        .collect()
}

fn part1(input: &[(Position, Velocity)], max_x: i32, max_y: i32) -> i32 {
    let mut top_left = 0;
    let mut top_right = 0;
    let mut bottom_left = 0;
    let mut bottom_right = 0;

    let mid_x = max_x / 2;
    let mid_y = max_y / 2;

    for robot in input {
        let ((pos_x, pos_y), (vel_x, vel_y)) = robot;
        let x = (pos_x + 100 * vel_x).rem_euclid(max_x);
        let y = (pos_y + 100 * vel_y).rem_euclid(max_y);

        match (x < mid_x, x > mid_x, y < mid_y, y > mid_y) {
            (true, false, true, false) => top_left += 1,
            (true, false, false, true) => bottom_left += 1,
            (false, true, true, false) => top_right += 1,
            (false, true, false, true) => bottom_right += 1,
            _ => (),
        }
    }

    top_left * top_right * bottom_left * bottom_right
}

#[allow(dead_code)]
fn print_map(robots: &[Position], max_x: i32, max_y: i32) {
    let robots_set = robots.iter().copied().collect::<HashSet<Position>>();
    for y in 0..max_y {
        for x in 0..max_x {
            print!(
                "{}",
                if robots_set.contains(&(x, y)) {
                    '*'
                } else {
                    '.'
                }
            );
        }
        println!();
    }
}

fn part2(input: &[(Position, Velocity)], max_x: i32, max_y: i32) -> i32 {
    let mut seconds = 0;
    let mut robots = input.iter().map(|robot| robot.0).collect::<Vec<_>>();

    loop {
        seconds += 1;

        let mut robots_set = HashSet::new();
        for (i, (_, (vel_x, vel_y))) in input.iter().enumerate() {
            let (curr_x, curr_y) = robots[i];
            let new_position = ((curr_x + vel_x).rem_euclid(max_x), (curr_y + vel_y).rem_euclid(max_y));
            robots[i] = new_position;
            robots_set.insert(new_position);
        }

        if robots_set.len() == robots.len() {
            // print_map(&robots, max_x, max_y);
            return seconds;
        }
    }
}

fn main() {
    let sample_text = helpers::sample_file!();
    let sample_input = parse_input(sample_text);
    let sample_answer1 = part1(&sample_input, 11, 7);
    assert_eq!(sample_answer1, 12);

    let text = helpers::input_file!();
    let input = parse_input(text);
    let answer1 = part1(&input, 101, 103);
    println!("{answer1}");
    let answer2 = part2(&input, 101, 103);
    println!("{answer2}");
}
