use helpers::position::Position;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

fn parse_input(text: String) -> Vec<Position> {
    text.lines()
        .map(|line| {
            let parts = line.split(',').collect::<Vec<&str>>();
            Position::new(parts[0].parse().unwrap(), parts[1].parse().unwrap())
        })
        .collect()
}

#[allow(dead_code)]
fn print_map(positions: &[Position], max_x: i32, max_y: i32) {
    let positions_set = positions.iter().copied().collect::<HashSet<Position>>();
    for y in 0..max_y {
        for x in 0..max_x {
            print!(
                "{}",
                if positions_set.contains(&Position::new(x, y)) {
                    '#'
                } else {
                    '.'
                }
            );
        }
        println!();
    }
}

#[derive(Eq, PartialEq, Debug)]
struct State {
    position: Position,
    steps_so_far: usize,
    manhattan_distance: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_h = self.manhattan_distance + self.steps_so_far;
        let other_h = other.manhattan_distance + other.steps_so_far;
        other_h.cmp(&self_h)
    }
}

fn manhattan_distance(curr_position: Position, end_position: Position) -> usize {
    end_position.get_urow() - curr_position.get_urow() + end_position.get_ucol()
        - curr_position.get_ucol()
}

fn a_star(
    obstacles: &[Position],
    num_obstacles_to_consider: usize,
    end_position: Position,
) -> Option<usize> {
    let max_rows = end_position.get_urow() + 1;
    let max_cols = end_position.get_ucol() + 1;

    let obstacles_to_consider = &obstacles[..num_obstacles_to_consider];
    // print_map(obstacles_to_consider, max_rows as i32, max_cols as i32);

    let starting_position = Position::new(0, 0);
    let neighbours = [
        Position::new(-1, 0),
        Position::new(1, 0),
        Position::new(0, -1),
        Position::new(0, 1),
    ];

    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(State {
        position: starting_position,
        steps_so_far: 0,
        manhattan_distance: manhattan_distance(starting_position, end_position),
    });
    let mut visited = HashSet::new();

    while let Some(curr_state) = priority_queue.pop() {
        if curr_state.position == end_position {
            return Some(curr_state.steps_so_far);
        }
        if visited.contains(&curr_state.position) {
            continue;
        }

        for direction in neighbours {
            let neighbour = curr_state.position + direction;
            if !neighbour.is_within_bounds(max_rows, max_cols) {
                continue;
            }
            if visited.contains(&neighbour) {
                continue;
            }
            if obstacles_to_consider.contains(&neighbour) {
                continue;
            }
            priority_queue.push(State {
                position: neighbour,
                steps_so_far: curr_state.steps_so_far + 1,
                manhattan_distance: manhattan_distance(neighbour, end_position),
            });
        }
        visited.insert(curr_state.position);
    }

    None
}

fn part1(
    obstacles: &[Position],
    num_obstacles_to_consider: usize,
    end_position: Position,
) -> usize {
    match a_star(obstacles, num_obstacles_to_consider, end_position) {
        Some(answer) => answer,
        None => panic!("No solution found"),
    }
}

fn part2(obstacles: &[Position], end_position: Position) -> String {
    let mut lo = 0;
    let mut hi = obstacles.len();

    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        match a_star(obstacles, mid, end_position) {
            Some(_) => lo = mid,
            None => match a_star(obstacles, mid - 1, end_position) {
                Some(_) => {
                    let last_obstacle = obstacles[mid - 1];
                    return format!("{},{}", last_obstacle.get_row(), last_obstacle.get_col());
                }
                None => hi = mid,
            },
        }
    }

    panic!("No solution found");
}

fn main() {
    let sample_text = helpers::sample_file!();
    let sample_input = parse_input(sample_text);
    let end_position = Position::new(6, 6);
    let sample_answer1 = part1(&sample_input, 12, end_position);
    assert_eq!(sample_answer1, 22);
    let sample_answer2 = part2(&sample_input, end_position);
    assert_eq!(sample_answer2, "6,1");

    let text = helpers::input_file!();
    let input = parse_input(text);
    let end_position = Position::new(70, 70);
    let answer1 = part1(&input, 1024, end_position);
    println!("{answer1}");
    let answer2 = part2(&input, end_position);
    println!("{answer2}");
}
