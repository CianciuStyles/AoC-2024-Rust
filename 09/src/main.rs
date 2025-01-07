use std::cmp::Ordering;

fn parse_input(input: String) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn create_disk_map_blocks(values: &[u32]) -> Vec<Option<u32>> {
    let mut disk_map = Vec::new();
    let mut free_space = false;
    let mut curr_id = 0;

    for value in values {
        for _ in 0..*value {
            if free_space {
                disk_map.push(None);
            } else {
                disk_map.push(Some(curr_id));
            }
        }
        free_space = !free_space;
        if free_space {
            curr_id += 1;
        }
    }

    disk_map
}
fn defragment_blocks(disk_map: &mut [Option<u32>]) {
    let mut left = 0;
    while disk_map[left].is_some() {
        left += 1;
    }
    let mut right = disk_map.len() - 1;
    while disk_map[right].is_none() {
        right -= 1;
    }

    while left < right {
        disk_map.swap(left, right);
        while disk_map[left].is_some() {
            left += 1;
        }
        while disk_map[right].is_none() {
            right -= 1;
        }
    }
}

fn part1(values: &[u32]) -> u64 {
    let mut disk_map = create_disk_map_blocks(values);
    defragment_blocks(&mut disk_map);

    disk_map
        .iter()
        .enumerate()
        .map(|(index, maybe_id)| u64::from(maybe_id.or(Some(0)).unwrap()) * index as u64)
        .sum()
}

fn create_disk_map_files(values: &[u32]) -> Vec<(Option<u32>, usize)> {
    let mut disk_map = Vec::new();
    let mut free_space = false;
    let mut curr_id = 0;

    for value in values {
        if free_space {
            if *value > 0 {
                disk_map.push((None, *value as usize));
            };
        } else {
            if *value > 0 {
                disk_map.push((Some(curr_id), *value as usize));
            };
            curr_id += 1;
        }
        free_space = !free_space;
    }

    disk_map
}

fn defragment_files(disk_map: &mut Vec<(Option<u32>, usize)>) {
    let mut right = disk_map.len() - 1;
    while let (None, _) = disk_map[right] {
        right -= 1;
    }
    let file_to_place = disk_map[right];
    let mut curr_id = file_to_place.0.unwrap();

    while curr_id > 0 {
        right = disk_map.len() - 1;
        loop {
            match disk_map.get(right) {
                None => break,
                Some(position) => match position {
                    (Some(id), _) if curr_id == *id => break,
                    _ => right -= 1,
                },
            }
        }
        let file_size = disk_map[right].1;

        let mut left = 0;
        loop {
            match disk_map[left] {
                (Some(_), _) => left += 1,
                (None, free_space) => match free_space.cmp(&file_size) {
                    Ordering::Less => {
                        left += 1;
                        if left >= right {
                            break;
                        }
                    }
                    Ordering::Equal => break,
                    Ordering::Greater => {
                        disk_map[left].1 = file_size;
                        disk_map.insert(left + 1, (None, free_space - file_size));
                        right += 1;
                        break;
                    }
                },
            }
        }

        if left < right {
            disk_map.swap(left, right);
        }

        curr_id -= 1;
    }
}

fn part2(values: &[u32]) -> u64 {
    let mut disk_map = create_disk_map_files(values);
    defragment_files(&mut disk_map);

    let mut curr_id = 0;
    disk_map
        .iter()
        .map(|(maybe_id, size)| {
            let result = match maybe_id {
                None => 0,
                Some(id) => (0..*size)
                    .map(|i| u64::from(*id) * (curr_id + i) as u64)
                    .sum(),
            };
            curr_id += size;
            result
        })
        .sum()
}

fn main() {
    let sample_text = helpers::sample_file!();
    let sample_input = parse_input(sample_text);
    let sample_answer1 = part1(&sample_input);
    assert_eq!(sample_answer1, 1928);
    let sample_answer2 = part2(&sample_input);
    assert_eq!(sample_answer2, 2858);

    let text = helpers::input_file!();
    let input = parse_input(text);
    let answer1 = part1(&input);
    println!("{answer1}");
    let answer2 = part2(&input);
    println!("{answer2}");
}
