type Lock = [u32; 5];
type Key = [u32; 5];
type Input = (Vec<Lock>, Vec<Key>);

fn parse_schematics(schematics: &[&str], vector: &mut Vec<[u32; 5]>) {
    let mut heights = [0; 5];

    for row in &schematics[1..] {
        for (i, char) in row.chars().enumerate() {
            heights[i] += u32::from(char == '#');
        }
    }

    vector.push(heights);
}

fn parse_input(text: String) -> Input {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    text.split("\n\n").for_each(|schematics| {
        let rows: Vec<&str> = schematics.lines().collect();
        match rows.first() {
            Some(&"#####") => {
                parse_schematics(&rows, &mut locks);
            }
            Some(&".....") => {
                parse_schematics(&rows, &mut keys);
            }
            _ => panic!("Unexpected input"),
        }
    });

    (locks, keys)
}

fn part1((locks, keys): &Input) -> u32 {
    let mut result = 0;

    for lock in locks {
        for key in keys {
            result += u32::from((0..5).all(|i| lock[i] + key[i] < 7));
        }
    }

    result
}

fn main() {
    let sample_text = helpers::sample_file!();
    let sample_input = parse_input(sample_text);
    let sample_answer1 = part1(&sample_input);
    assert_eq!(sample_answer1, 3);

    let text = helpers::input_file!();
    let input = parse_input(text);
    let answer1 = part1(&input);
    println!("{answer1}");
}
