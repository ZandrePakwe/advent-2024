use std::{fs, usize};

fn read_input() -> Vec<String> {
    let input_text = fs::read_to_string("src/day9/input.txt").expect("Error reading input");

    let mut files = vec![];
    let mut empty_space = vec![];
    for index in 0..input_text.len() {
        let current_char = input_text.chars().nth(index).unwrap();
        let number = current_char.to_digit(10).unwrap();
        if index % 2 == 0 {
            files.push(number);
        } else {
            empty_space.push(number);
        }
    }
    let mut parsed_string = vec![];

    for index in 0..files.len() {
        for _ in 0..files[index] {
            parsed_string.push(format!("{index}"));
        }
        if empty_space.len() > index {
            for _ in 0..empty_space[index] {
                parsed_string.push(".".to_string());
            }
        }
    }
    return parsed_string;
}

pub fn day_9_part_1() {
    let mut input = read_input();
    compact_contents(&mut input);
    let checksum = calculate_checksum(input);
    println!("{checksum}")
}

pub fn day_9_part_2() {
    let mut input = read_input();
    compact_files(&mut input);
    let checksum = calculate_checksum(input);
    println!("{checksum}")
}

fn compact_contents(input: &mut Vec<String>) {
    let mut index = 1;
    loop {
        let index_of_first_empty_space = input.iter().position(|character| character == ".");

        if let Some(index_of_first_empty_space) = index_of_first_empty_space {
            if input[index_of_first_empty_space..]
                .iter()
                .all(|character| *character == ".")
            {
                break;
            }
            let index_of_last_non_empty_space = &input.len() - index;
            input.swap(index_of_first_empty_space, index_of_last_non_empty_space);
            index += 1;
        } else {
            break;
        }
    }
}

fn compact_files(input: &mut Vec<String>) {
    let mut last_file_id = input
        .iter()
        .filter(|id| **id != ".")
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    loop {
        if last_file_id == 0 {
            break;
        }

        let last_file_size = input
            .iter()
            .filter(|id| **id == last_file_id.to_string())
            .count();

        let last_file_position = input
            .iter()
            .position(|id| *id == last_file_id.to_string())
            .unwrap();

        for index in 0..last_file_position {
            let slice = &input[index..index + last_file_size];
            if slice.iter().all(|id| id == ".") {
                for swop_index in 0..last_file_size {
                    input.swap(index + swop_index, last_file_position + swop_index);
                }

                break;
            }
        }
        last_file_id -= 1;
    }
}

fn calculate_checksum(input: Vec<String>) -> usize {
    let mut checksum = 0;
    for index in 0..input.len() {
        if let Ok(value) = input[index].parse::<usize>() {
            checksum += index * value;
        }
    }
    return checksum;
}
