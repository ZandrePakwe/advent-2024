use std::{fs, usize};

fn read_input() -> Vec<CalibrationSet> {
    let text = fs::read_to_string("src/day7/input.txt").expect("Input file not found for day 7");

    return text
        .split('\n')
        .map(|line| {
            let parts = line.split(":").collect::<Vec<&str>>();

            let answer = parts[0].parse::<usize>().expect("Incorrect Input format");

            let values = parts[1]
                .split(" ")
                .filter(|value| *value != "")
                .map(|value| value.parse::<usize>().expect("Incorrect Input Format"))
                .collect::<Vec<usize>>();

            return CalibrationSet { answer, values };
        })
        .collect::<Vec<CalibrationSet>>();
}

struct CalibrationSet {
    answer: usize,
    values: Vec<usize>,
}

pub fn day_7_part_1() {
    let input = read_input();

    let mut correct_calibrations = vec![];
    for set in input {
        if is_valid_calibration(&set.values, &set.answer, 2) {
            correct_calibrations.push(set.answer);
        }
    }

    println!(
        "There are {} correct calibrations, with a total of {:?}",
        correct_calibrations.len(),
        correct_calibrations.iter().sum::<usize>()
    )
}

pub fn day_7_part_2() {
    let input = read_input();

    let mut correct_calibrations = vec![];
    for set in input {
        if is_valid_calibration(&set.values, &set.answer, 3) {
            correct_calibrations.push(set.answer);
        }
    }

    println!(
        "There are {} correct calibrations, with a total of {:?}",
        correct_calibrations.len(),
        correct_calibrations.iter().sum::<usize>()
    )
}

fn is_valid_calibration(values: &Vec<usize>, answer: &usize, base: usize) -> bool {
    let mut is_possible = false;

    let mut binary_number: usize = base.pow(values.len() as u32 - 1) - 1;

    let padding = to_base_n(binary_number, base).len();
    loop {
        let mut total = values[0];
        for index in 0..values.len() - 1 {
            match format!("{:0>1$}", to_base_n(binary_number, base), padding)
                .chars()
                .nth(index)
            {
                Some('2') => {
                    total = format!("{}{}", total, values[index + 1])
                        .parse()
                        .expect("Error with input")
                }
                Some('1') => total += values[index + 1],
                Some('0') => total *= values[index + 1],
                Some(_) => continue,
                None => continue,
            }
            if total > *answer {
                break;
            }

        }
        if total == *answer {
            is_possible = true;
            break;
        }
        if binary_number == 0 {
            break;
        }
        binary_number -= 1;
    }

    return is_possible;
}

fn to_base_n(mut num: usize, base: usize) -> String {
    if num == 0 {
        return "0".to_string();
    }

    let mut digits = Vec::new();
    while num > 0 {
        digits.push((num % base).to_string());
        num /= base;
    }

    digits.reverse();
    digits.concat()
}
