use helpers::direction::Direction;
use helpers::position::Position;
use helpers::position_with_direction::PositionWithDirection;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

type Input = (Position, Position, HashSet<Position>);

fn parse_input(text: String) -> Input {
    let mut start = Position::new(-1, -1);
    let mut end = Position::new(-1, -1);
    let mut walls = HashSet::new();

    for (r, row) in text.lines().enumerate() {
        for (c, cell) in row.chars().enumerate() {
            match cell {
                'S' => start = Position::from_usize(r, c),
                'E' => end = Position::from_usize(r, c),
                '#' => {
                    walls.insert(Position::from_usize(r, c));
                }
                _ => {}
            }
        }
    }

    (start, end, walls)
}

#[derive(Eq, PartialEq)]
struct State {
    position_with_direction: PositionWithDirection,
    score: u32,
    estimated_distance: u32,
    path: HashSet<Position>,
}

impl PartialOrd<Self> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_h = self.score + self.estimated_distance;
        let other_h = other.score + other.estimated_distance;

        other_h.cmp(&self_h)
    }
}

fn manhattan(curr: Position, end: Position) -> u32 {
    u32::try_from(curr.get_row() - end.get_row().abs() + (curr.get_col() - end.get_col()).abs())
        .expect("Invalid manhattan distance")
}

fn handle_move_forwards(
    priority_queue: &mut BinaryHeap<State>,
    curr_state: &State,
    end: Position,
    walls: &HashSet<Position>,
) {
    let curr_position = curr_state.position_with_direction.get_position();
    let curr_direction = curr_state.position_with_direction.get_direction();

    let next_step = match curr_direction {
        Direction::NORTH => curr_position.up(),
        Direction::EAST => curr_position.right(),
        Direction::SOUTH => curr_position.down(),
        Direction::WEST => curr_position.left(),
    };
    if !walls.contains(&next_step) {
        let mut new_path = curr_state.path.clone();
        new_path.insert(curr_position);
        
        priority_queue.push(State {
            position_with_direction: PositionWithDirection::from_position_and_direction(
                next_step,
                curr_direction,
            ),
            score: curr_state.score + 1,
            estimated_distance: manhattan(next_step, end),
            path: new_path,
        });
    }
}

fn handle_rotate(priority_queue: &mut BinaryHeap<State>, curr_state: &State) {
    let curr_position = curr_state.position_with_direction.get_position();
    let curr_direction = curr_state.position_with_direction.get_direction();

    let (next_direction_clockwise, next_direction_counterclockwise) = match curr_direction {
        Direction::NORTH => (Direction::EAST, Direction::WEST),
        Direction::EAST => (Direction::SOUTH, Direction::NORTH),
        Direction::SOUTH => (Direction::WEST, Direction::EAST),
        Direction::WEST => (Direction::NORTH, Direction::SOUTH),
    };
    priority_queue.push(State {
        position_with_direction: PositionWithDirection::from_position_and_direction(
            curr_position,
            next_direction_clockwise,
        ),
        score: curr_state.score + 1000,
        estimated_distance: curr_state.estimated_distance,
        path: curr_state.path.clone(),
    });
    priority_queue.push(State {
        position_with_direction: PositionWithDirection::from_position_and_direction(
            curr_position,
            next_direction_counterclockwise,
        ),
        score: curr_state.score + 1000,
        estimated_distance: curr_state.estimated_distance,
        path: curr_state.path.clone(),
    });
}

fn part1((start, end, walls): &Input) -> u32 {
    let mut priority_queue = BinaryHeap::new();
    let mut visited = HashSet::<PositionWithDirection>::new();

    let start_state = State {
        position_with_direction: PositionWithDirection::from_position_and_direction(
            *start,
            Direction::EAST,
        ),
        score: 0,
        estimated_distance: manhattan(*start, *end),
        path: HashSet::new(),
    };
    priority_queue.push(start_state);
    while !priority_queue.is_empty() {
        let curr_state = priority_queue.pop().unwrap();

        let curr_position_with_direction = curr_state.position_with_direction;
        let curr_position = curr_position_with_direction.get_position();

        if curr_position == *end {
            return curr_state.score;
        };

        if visited.contains(&curr_position_with_direction) {
            continue;
        }
        visited.insert(curr_position_with_direction);

        handle_move_forwards(&mut priority_queue, &curr_state, *end, walls);
        handle_rotate(&mut priority_queue, &curr_state);
    }

    panic!("No solution found!");
}

fn part2((start, end, walls): &Input) -> u32 {
    let mut priority_queue = BinaryHeap::new();
    let mut visited = HashMap::<PositionWithDirection, u32>::new();
    let mut min_score = None;
    let mut best_tiles = HashSet::new();

    let start_state = State {
        position_with_direction: PositionWithDirection::from_position_and_direction(
            *start,
            Direction::EAST,
        ),
        score: 0,
        estimated_distance: manhattan(*start, *end),
        path: HashSet::new(),
    };
    priority_queue.push(start_state);

    while !priority_queue.is_empty() {
        let curr_state = priority_queue.pop().unwrap();
        let curr_position = curr_state.position_with_direction.get_position();

        if min_score.is_some() && curr_state.score > min_score.unwrap() {
            break;
        }
        if curr_position == *end {
            min_score = Some(curr_state.score);
            for tile in curr_state.path {
                best_tiles.insert(tile);
            }
            best_tiles.insert(*end);
            continue;
        }

        match visited.get(&curr_state.position_with_direction) {
            Some(prev_score) => {
                if *prev_score < curr_state.score {
                    continue;
                }
            }
            None => {
                visited.insert(curr_state.position_with_direction, curr_state.score);
            }
        }

        handle_move_forwards(&mut priority_queue, &curr_state, *end, walls);
        handle_rotate(&mut priority_queue, &curr_state);
    }

    u32::try_from(best_tiles.len()).expect("Invalid result!")
}

fn main() {
    let sample_text = helpers::sample_file!();
    let sample_input = parse_input(sample_text);
    let sample_answer1 = part1(&sample_input);
    assert_eq!(sample_answer1, 11048);
    let sample_answer2 = part2(&sample_input);
    assert_eq!(sample_answer2, 64);

    let text = helpers::input_file!();
    let input = parse_input(text);
    let answer1 = part1(&input);
    println!("{answer1}");
    let answer2 = part2(&input);
    println!("{answer2}");
}
