use std::fs;

use regex::Regex;

pub fn day_3() {
    let input = fs::read_to_string("src/day3/input.txt").expect("File not found");

    let find_instructions_regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let find_numbers_regex = Regex::new(r"\d{1,3}").unwrap();

    // Find all matches
    let mut addition_result = 0;
    for multiply_instruction_match in find_instructions_regex.find_iter(&*input) {
        let mut multiply_result = 1;
        for number_match in find_numbers_regex.find_iter(multiply_instruction_match.as_str()) {
            let number = number_match
                .as_str()
                .parse::<i64>()
                .expect("incorrect number");
            multiply_result *= number;
        }
        // println!(
        //     "Found: {} = {}",
        //     multiply_instruction_match.as_str(),
        //     multiply_result
        // );

        addition_result += multiply_result;
    }

    println!("Total answer is: {}", addition_result);
}

pub fn day_3_part_2() {
    let mut input = fs::read_to_string("src/day3/input.txt").expect("File not found");

    let mut flag = true;

    while flag {
        if let Some(position_of_dont) = input.find("don't()") {
            let (_, rest_of_input) = input.split_at(position_of_dont);
            let position_of_do = rest_of_input.find("do()").unwrap_or(rest_of_input.len());
            input.drain(position_of_dont..position_of_do + position_of_dont);
        } else {
            flag = false;
        }
    }

    let find_instructions_regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let find_numbers_regex = Regex::new(r"\d{1,3}").unwrap();

    // Find all matches
    let mut addition_result = 0;
    for multiply_instruction_match in find_instructions_regex.find_iter(&*input) {
        let mut multiply_result = 1;
        for number_match in find_numbers_regex.find_iter(multiply_instruction_match.as_str()) {
            let number = number_match
                .as_str()
                .parse::<i64>()
                .expect("incorrect number");
            multiply_result *= number;
        }
        // println!(
        //     "Found: {} = {}",
        //     multiply_instruction_match.as_str(),
        //     multiply_result
        // );

        addition_result += multiply_result;
    }

    println!("Total answer is: {}", addition_result);
}
