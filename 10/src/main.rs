use helpers::position::Position;
use std::collections::HashSet;

type Map = Vec<Vec<u32>>;
type ScoresMap = Vec<Vec<HashSet<Position>>>;
type RatingsMap = Vec<Vec<u32>>;
type Input = (Map, HashSet<Position>, HashSet<Position>);

fn parse_input(input: &str) -> Input {
    let mut starts = HashSet::new();
    let mut ends = HashSet::new();
    let map = input
        .lines()
        .enumerate()
        .map(|(r, row)| {
            row.chars()
                .enumerate()
                .map(|(c, digit)| {
                    let height = digit.to_digit(10).unwrap();
                    if height == 0 {
                        starts.insert(Position::from_usize(r, c));
                    }
                    if height == 9 {
                        ends.insert(Position::from_usize(r, c));
                    }
                    height
                })
                .collect()
        })
        .collect();

    (map, starts, ends)
}

fn calculate_scores(map: &Map, ends: &HashSet<Position>) -> ScoresMap {
    let mut scores: ScoresMap = map
        .iter()
        .map(|row| row.iter().map(|_| HashSet::<Position>::new()).collect())
        .collect();

    for end in ends {
        scores[end.get_urow()][end.get_ucol()].insert(*end);
    }

    let mut current_height = 8;
    loop {
        for (r, row) in map.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                if *cell != current_height {
                    continue;
                }
                let current_position = Position::from_usize(r, c);
                for delta in [
                    Position::new(-1, 0),
                    Position::new(1, 0),
                    Position::new(0, -1),
                    Position::new(0, 1),
                ] {
                    let neighbour = current_position + delta;
                    if neighbour.is_within_bounds(map.len(), map[0].len())
                        && map[neighbour.get_urow()][neighbour.get_ucol()] == current_height + 1
                    {
                        scores[r][c] = scores[r][c]
                            .union(&scores[neighbour.get_urow()][neighbour.get_ucol()].clone())
                            .copied()
                            .collect();
                    }
                }
            }
        }
        match current_height.checked_sub(1) {
            Some(next_height) => current_height = next_height,
            None => break,
        }
    }

    scores
}

fn part1((map, starts, ends): &Input) -> usize {
    let scores = calculate_scores(map, ends);

    starts
        .iter()
        .map(|start| scores[start.get_urow()][start.get_ucol()].len())
        .sum()
}

fn calculate_ratings(map: &Map, ends: &HashSet<Position>) -> RatingsMap {
    let mut ratings: Map = map
        .iter()
        .map(|row| row.iter().map(|_| 0).collect())
        .collect();
    for end in ends {
        ratings[end.get_urow()][end.get_ucol()] = 1;
    }

    let mut current_height = 8;
    loop {
        for (r, row) in map.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                if *cell != current_height {
                    continue;
                }
                let current_position = Position::from_usize(r, c);
                for delta in [
                    Position::new(-1, 0),
                    Position::new(1, 0),
                    Position::new(0, -1),
                    Position::new(0, 1),
                ] {
                    let neighbour = current_position + delta;
                    if neighbour.is_within_bounds(map.len(), map[0].len())
                        && map[neighbour.get_urow()][neighbour.get_ucol()] == current_height + 1
                    {
                        ratings[r][c] += ratings[neighbour.get_urow()][neighbour.get_ucol()];
                    }
                }
            }
        }
        match current_height.checked_sub(1) {
            Some(next_height) => current_height = next_height,
            None => break,
        }
    }

    ratings
}

fn part2((map, starts, ends): &Input) -> u32 {
    let ratings = calculate_ratings(map, ends);

    starts
        .iter()
        .map(|start| ratings[start.get_urow()][start.get_ucol()])
        .sum()
}

fn main() {
    let sample_text = helpers::sample_file!();
    let sample_input = parse_input(&sample_text);
    let sample_answer1 = part1(&sample_input);
    assert_eq!(sample_answer1, 36);
    let sample_answer2 = part2(&sample_input);
    assert_eq!(sample_answer2, 81);

    let text = helpers::input_file!();
    let input = parse_input(&text);
    let answer1 = part1(&input);
    println!("{answer1}");
    let answer2 = part2(&input);
    println!("{answer2}");
}
