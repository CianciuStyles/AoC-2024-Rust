use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

type Position = (usize, usize);
type Mappings = HashMap<Direction, (isize, isize, Direction)>;
type Input = (Position, HashSet<Position>, usize, usize);

fn parse_input(input: &str) -> Input {
    let mut starting_position = (0, 0);
    let mut obstacles = HashSet::new();
    let mut num_rows = 0;
    let mut num_cols = 0;

    for (row, line) in input.lines().enumerate() {
        num_rows = row;
        for (col, cell) in line.chars().enumerate() {
            num_cols = col;
            match cell {
                '#' => {
                    obstacles.insert((row, col));
                }
                '^' => {
                    starting_position = (row, col);
                }
                _ => {}
            }
        }
    }

    (starting_position, obstacles, num_rows, num_cols)
}

fn simulate_patrol(
    (starting_position, obstacles, num_rows, num_cols): &Input,
    mappings: &Mappings,
    additional_obstacle: Option<Position>,
) -> (bool, HashSet<Position>) {
    let mut current_position = *starting_position;
    let mut current_direction = Direction::Up;
    let mut visited_positions = HashSet::new();
    let mut visited_positions_with_direction = HashSet::new();

    while current_position.0 < *num_rows && current_position.1 < *num_cols {
        if visited_positions_with_direction.contains(&(current_position, current_direction)) {
            return (true, HashSet::new());
        }

        let (dr, dc, new_direction) = mappings.get(&current_direction).unwrap();
        let Some(new_row) = current_position.0.checked_add_signed(*dr) else {
            break;
        };
        let Some(new_col) = current_position.1.checked_add_signed(*dc) else {
            break;
        };
        let new_position: Position = (new_row, new_col);
        if obstacles.contains(&new_position)
            || additional_obstacle.is_some() && additional_obstacle.unwrap() == new_position
        {
            current_direction = *new_direction;
        } else {
            visited_positions.insert(current_position);
            visited_positions_with_direction.insert((current_position, current_direction));
            current_position = new_position;
        }
    }

    (false, visited_positions)
}
fn part1(input: &Input, mappings: &Mappings) -> usize {
    match simulate_patrol(input, mappings, None) {
        (true, _) => 0,
        (false, path) => path.len() + 1,
    }
}

fn part2(input: &Input, mappings: &Mappings) -> u32 {
    match simulate_patrol(input, mappings, None) {
        (true, _) => 0,
        (false, path) => {
            u32::try_from(
                path.iter()
                    .filter(|possible_new_obstacle| {
                        simulate_patrol(input, mappings, Some(**possible_new_obstacle)).0
                    })
                    .count(),
            )
            .unwrap_or(0)
                + 1
        }
    }
}

fn main() {
    let mappings: Mappings = HashMap::from([
        (Direction::Up, (-1, 0, Direction::Right)),
        (Direction::Right, (0, 1, Direction::Down)),
        (Direction::Down, (1, 0, Direction::Left)),
        (Direction::Left, (0, -1, Direction::Up)),
    ]);

    let sample_text = helpers::sample_file!();
    let sample_input = parse_input(&sample_text);
    let sample_answer1 = part1(&sample_input, &mappings);
    assert_eq!(sample_answer1, 41);
    let sample_answer2 = part2(&sample_input, &mappings);
    assert_eq!(sample_answer2, 6);

    let text = helpers::input_file!();
    let input = parse_input(&text);
    let answer1 = part1(&input, &mappings);
    println!("{answer1}");
    let answer2 = part2(&input, &mappings);
    println!("{answer2}");
}
