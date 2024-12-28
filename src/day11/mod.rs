use std::{collections::HashMap, fs};

fn read_input() -> Vec<usize> {
    let input = fs::read_to_string("src/day11/input.txt").expect("no Input for day 11");

    return input
        .split(" ")
        .map(|number| number.parse::<usize>().unwrap())
        .collect();
}

pub fn day_11_part_1() {
    let mut input = read_input();

    let number_of_blinks = 25;

    for _blink in 0..number_of_blinks {
        get_next_sequence(&mut input);
    }

    println!("{} stones after {} blinks", input.len(), number_of_blinks);
}

pub fn day_11_part_2() {
    let input_array = read_input();

    let mut input = HashMap::new();

    for number in input_array {
        *input.entry(number).or_insert(0) += 1;
    }

    let number_of_blinks = 75;

    for _ in 0..number_of_blinks {
        get_next_sequence_part_2(&mut input);
    }
    println!(
        "{} stones after {} blinks",
        input.iter().map(|(_, count)| count).sum::<usize>(),
        number_of_blinks
    );
}

fn get_next_sequence(input: &mut Vec<usize>) {
    let mut new_sequence: Vec<usize> = vec![];
    for number in input.clone() {
        if number == 0 {
            new_sequence.push(1);
            continue;
        }

        if number.to_string().len() % 2 == 0 {
            let number = number.to_string();

            let (number_1, number_2) = number.split_at(number.len() / 2);

            new_sequence.push(number_1.parse().unwrap());
            new_sequence.push(number_2.parse().unwrap());

            continue;
        }

        new_sequence.push(number * 2024);
    }

    *input = new_sequence;
}

fn get_next_sequence_part_2(input: &mut HashMap<usize, usize>) {
    let mut new_map: HashMap<usize, usize> = HashMap::new();

    for (number, count) in input.clone() {
        if number == 0 {
            *new_map.entry(1).or_insert(0) += count;
            continue;
        }

        if number.to_string().len() % 2 == 0 {
            let number = number.to_string();

            let (number_1, number_2) = number.split_at(number.len() / 2);

            *new_map.entry(number_1.parse().unwrap()).or_insert(0) += count;
            *new_map.entry(number_2.parse().unwrap()).or_insert(0) += count;

            continue;
        }

        *new_map.entry(number * 2024).or_insert(0) += count;
    }
    
    *input = new_map;
}
