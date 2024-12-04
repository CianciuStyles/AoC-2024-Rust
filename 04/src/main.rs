fn parse_input(text: String) -> Vec<Vec<char>> {
    text.split("\n").map(|l| l.chars().collect()).collect()
}

fn search_word(
    input: &[Vec<char>],
    r: usize,
    c: usize,
    word_to_find: &Vec<char>,
    dr: isize,
    dc: isize,
) -> bool {
    let max_r = input.len();
    let max_c = input[0].len();

    word_to_find.iter().enumerate().all(|(i, curr_char)| {
        let new_r = match r.checked_add_signed(i as isize * dr) {
            Some(new_r) if new_r < max_r => new_r,
            Some(_) => return false,
            None => return false,
        };

        let new_c = match c.checked_add_signed(i as isize * dc) {
            Some(new_c) if new_c < max_c => new_c,
            Some(_) => return false,
            None => return false,
        };

        input[new_r][new_c] == *curr_char
    })
}

fn search_with_deltas(
    input: &[Vec<char>],
    r: usize,
    c: usize,
    word_to_find: &Vec<char>,
    dr: isize,
    dc: isize,
) -> bool {
    search_word(input, r, c, word_to_find, dr, dc)
        || search_word(
            input,
            r,
            c,
            &word_to_find.iter().copied().rev().collect(),
            dr,
            dc,
        )
}

fn search_horizontally(input: &[Vec<char>], r: usize, c: usize, word_to_find: &Vec<char>) -> bool {
    search_with_deltas(input, r, c, word_to_find, 0, 1)
}

fn search_vertically(input: &[Vec<char>], r: usize, c: usize, word_to_find: &Vec<char>) -> bool {
    search_with_deltas(input, r, c, word_to_find, 1, 0)
}

fn search_diagonally_backwards(
    input: &[Vec<char>],
    r: usize,
    c: usize,
    word_to_find: &Vec<char>,
) -> bool {
    search_with_deltas(input, r, c, word_to_find, 1, -1)
}

fn search_diagonally_forwards(
    input: &[Vec<char>],
    r: usize,
    c: usize,
    word_to_find: &Vec<char>,
) -> bool {
    search_with_deltas(input, r, c, word_to_find, 1, 1)
}

fn part1(input: &[Vec<char>]) -> isize {
    let mut result = 0;
    let word_to_find = "XMAS";
    let chars: Vec<char> = word_to_find.chars().collect();

    (0..input.len()).for_each(|r| {
        (0..input[r].len()).for_each(|c| {
            if search_horizontally(input, r, c, &chars) {
                result += 1;
            }
            if search_vertically(input, r, c, &chars) {
                result += 1;
            }
            if search_diagonally_forwards(input, r, c, &chars) {
                result += 1;
            }
            if search_diagonally_backwards(input, r, c, &chars) {
                result += 1;
            }
        });
    });

    result
}

fn part2(input: &[Vec<char>]) -> isize {
    let mut result = 0;
    let word_to_find = "MAS";
    let chars = &word_to_find.chars().collect();
    let word_len = word_to_find.len();

    (0..input.len()).for_each(|r| {
        (0..input[r].len()).for_each(|c| {
            if search_diagonally_forwards(input, r, c, chars)
                && search_diagonally_backwards(input, r, c + word_len - 1, chars)
            {
                result += 1
            }
        });
    });

    result
}

fn main() {
    let sample_text = helpers::sample_file!();
    let sample_input = parse_input(sample_text);
    let sample_answer1 = part1(&sample_input);
    assert_eq!(sample_answer1, 18);
    let sample_answer2 = part2(&sample_input);
    assert_eq!(sample_answer2, 9);

    let text = helpers::input_file!();
    let input = parse_input(text);
    let answer1 = part1(&input);
    println!("{}", answer1);
    let answer2 = part2(&input);
    println!("{}", answer2);
}
