type Input = (u64, u64, u64, Vec<u64>);
fn parse_input(text: String) -> Input {
    let parts: Vec<&str> = text.split("\n\n").collect();

    let registers: Vec<u64> = parts[0]
        .lines()
        .map(|line| {
            let line_parts: Vec<&str> = line.split(": ").collect();
            line_parts[1]
                .parse::<u64>()
                .expect("Invalid register value")
        })
        .collect();

    let program: Vec<&str> = parts[1].split(": ").collect();
    let instructions = program[1]
        .split(',')
        .map(|instruction| instruction.parse::<u64>().expect("Invalid opcode"))
        .collect();

    (registers[0], registers[1], registers[2], instructions)
}

fn combo(a: u64, b: u64, c: u64, combo: u64) -> u64 {
    match combo {
        0..=3 => combo,
        4 => a,
        5 => b,
        6 => c,
        _ => panic!("Invalid operand"),
    }
}

fn execute_program(mut a: u64, mut b: u64, mut c: u64, instructions: &[u64]) -> Vec<u64> {
    let mut output = Vec::new();

    let mut pc = 0;
    while pc < instructions.len() {
        if pc + 1 == instructions.len() {
            break;
        }
        let operand = instructions[pc + 1];
        match instructions[pc] {
            0 => a = a >> combo(a, b, c, operand),
            1 => b ^= operand,
            2 => b = combo(a, b, c, operand) & 0x7,
            3 => {
                if a != 0 {
                    pc = usize::try_from(operand).expect("Invalid destination");
                    continue;
                }
            }
            4 => b ^= c,
            5 => output.push(combo(a, b, c, operand) & 0x7),
            6 => b = a >> combo(a, b, c, operand),
            7 => c = a >> combo(a, b, c, operand),
            _ => panic!("Invalid instruction"),
        }
        pc += 2;
    }

    output
}

fn part1((a, b, c, instructions): &Input) -> String {
    execute_program(*a, *b, *c, instructions)
        .into_iter()
        .map(|digit| digit.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn search_answer(instructions: &Vec<u64>, answer_so_far: u64) -> Option<u64> {
    for a in 0..8 {
        let candidate_a = answer_so_far << 3 | a;
        let current_output = execute_program(candidate_a, 0, 0, instructions);

        if current_output == *instructions {
            return Some(candidate_a);
        } else if instructions.ends_with(&current_output) {
            if let Some(answer) = search_answer(instructions, candidate_a) {
                return Some(answer);
            }
        }
    }

    None
}

fn part2((_, _, _, instructions): &Input) -> u64 {
    match search_answer(instructions, 0) {
        Some(answer) => answer,
        None => panic!("No answer found"),
    }
}

fn main() {
    let sample_text = helpers::sample_file!();
    let sample_input = parse_input(sample_text);
    let sample_answer1 = part1(&sample_input);
    assert_eq!(sample_answer1, "4,6,3,5,6,3,5,2,1,0");

    let text = helpers::input_file!();
    let input = parse_input(text);
    let answer1 = part1(&input);
    println!("{answer1}");
    let answer2 = part2(&input);
    println!("{answer2}");
}
