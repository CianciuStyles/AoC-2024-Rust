use std::collections::{HashMap, HashSet};

use helpers::position::Position;

type Input = (HashMap<char, Vec<Position>>, usize, usize);

fn parse_input(text: String) -> Input {
    let mut frequencies = HashMap::new();
    let mut num_rows = 0;
    let mut num_cols = 0;

    for (r, row) in text.lines().enumerate() {
        num_rows = r;
        for (c, frequency) in row.chars().enumerate() {
            num_cols = c;

            if frequency != '.' {
                frequencies
                    .entry(frequency)
                    .or_insert(vec![])
                    .push(Position::from_usize(r, c));
            }
        }
    }

    (frequencies, num_rows + 1, num_cols + 1)
}

fn find_antinodes(
    positions: &[Position],
    num_rows: usize,
    num_cols: usize,
    consider_harmonics: bool,
) -> HashSet<Position> {
    let mut antinodes: HashSet<Position> = HashSet::new();

    for i in 0..positions.len() {
        for j in i + 1..positions.len() {
            let first = positions[i];
            let second = positions[j];

            if consider_harmonics {
                antinodes.insert(first);
                antinodes.insert(second);
            }

            let delta = Position::new(
                second.get_row() - first.get_row(),
                second.get_col() - first.get_col(),
            );

            let mut new_position = first - delta;
            while new_position.is_within_bounds(num_rows, num_cols) {
                antinodes.insert(new_position);
                if !consider_harmonics {
                    break;
                }
                new_position -= delta;
            }

            new_position = second + delta;
            while new_position.is_within_bounds(num_rows, num_cols) {
                antinodes.insert(new_position);
                if !consider_harmonics {
                    break;
                }
                new_position += delta;
            }
        }
    }

    antinodes
}

fn part1((frequencies, num_rows, num_cols): &Input) -> usize {
    frequencies
        .values()
        .flat_map(|positions| find_antinodes(positions, *num_rows, *num_cols, false))
        .collect::<HashSet<Position>>()
        .len()
}

fn part2((frequencies, num_rows, num_cols): &Input) -> usize {
    frequencies
        .values()
        .flat_map(|positions| find_antinodes(positions, *num_rows, *num_cols, true))
        .collect::<HashSet<Position>>()
        .len()
}

fn main() {
    let sample_input = parse_input(helpers::sample_file!());
    let sample_answer1 = part1(&sample_input);
    assert_eq!(sample_answer1, 14);
    let sample_answer2 = part2(&sample_input);
    assert_eq!(sample_answer2, 34);

    let input = parse_input(helpers::input_file!());
    let answer1 = part1(&input);
    println!("{answer1}");
    let answer2 = part2(&input);
    println!("{answer2}");
}
