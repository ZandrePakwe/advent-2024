use std::{
    fs,
    ops::Index,
    sync::{atomic::AtomicUsize, Arc},
    thread,
};

fn can_be_made_from_segments(segments: &Vec<String>, string_to_build: &str) -> bool {
    for segment in segments.clone() {
        if string_to_build.starts_with(&segment) {
            let (_, rest_of_string) = string_to_build.split_at(segment.len());
            if rest_of_string.is_empty() {
                return true;
            }

            let valid_segments = segments
                .iter()
                .map(|segment| segment.clone())
                .filter(|segment| rest_of_string.contains(segment))
                .collect::<Vec<String>>();

            if can_be_made_from_segments(&valid_segments, rest_of_string) {
                return true;
            }
        }
    }

    return false;
}

fn count_number_of_solutions(segments: &Vec<String>, string_to_build: &str) -> usize {
    let mut number_of_solutions = 0;
    for segment in segments.clone() {
        if string_to_build.starts_with(&segment) {
            let (_, rest_of_string) = string_to_build.split_at(segment.len());
            if rest_of_string.is_empty() {
                number_of_solutions += 1;
            }

            let valid_segments = segments
                .iter()
                .map(|segment| segment.clone())
                .filter(|segment| rest_of_string.contains(segment))
                .collect::<Vec<String>>();

            number_of_solutions += count_number_of_solutions(&valid_segments, rest_of_string);
        }
    }

    return number_of_solutions;
}

fn count_number_of_solutions_rewrite(segments: &Vec<String>, strings_to_build: Vec<&str>) -> usize {
    let mut total = 0;

    for string_to_build in strings_to_build {
        total += sort_towels(segments, string_to_build);
    }

    return total;
}

fn sort_towels(segments: &Vec<String>, string_to_build: &str) -> usize {
    let mut segments_used = vec![vec![]; string_to_build.len()];

    for segment in segments {
        if string_to_build.contains(segment) {
            let mut last_index = 0;

            while string_to_build[last_index..].contains(segment) {
                let index = string_to_build[last_index..].find(segment).unwrap() + last_index;

                segments_used[index].push(segment.clone());

                last_index = index + 1;
            }
        }
    }

    let mut possibilities = vec![0; string_to_build.len()];

    for index in (0..segments_used.len()).rev() {
        for possible_segments_at_index in &segments_used[index] {
            if index + possible_segments_at_index.len() < string_to_build.len() {
                possibilities[index] += possibilities[index + possible_segments_at_index.len()];
            } else {
                possibilities[index] += 1;
            }
            println!("{:?}", possibilities);
        }
    }

    if *possibilities.first().unwrap() > 0 {
    } else {
    }

    return *possibilities.first().unwrap();
}

fn read_input() -> (Vec<String>, Vec<String>) {
    let input = fs::read_to_string("src/day19/input.txt").expect("input for day 19 not present");

    let mut input = input.lines();

    let segments = input
        .next()
        .unwrap()
        .split(", ")
        .map(|element| element.to_string())
        .collect();

    input.next();

    let strings_to_build = input.map(|element| element.to_string()).collect();

    return (segments, strings_to_build);
}

pub fn day_19_part_1() {
    let (segments, strings_to_build) = read_input();

    let mut total_possible = 0;

    for string_to_build in strings_to_build {
        let segments = segments
            .clone()
            .iter()
            .map(|segment| segment.clone())
            .filter(|segment| string_to_build.contains(segment))
            .collect::<Vec<String>>();

        if can_be_made_from_segments(&segments, string_to_build.as_str()) {
            total_possible += 1;
        }
    }

    println!("total possible combinations: {}", total_possible)
}

pub fn day_19_part_2() {
    let (segments, strings_to_build) = read_input();

    let total = count_number_of_solutions_rewrite(
        &segments,
        strings_to_build.iter().map(|str| str.as_str()).collect(),
    );

    println!("{}", total);

    // let total_possible = Arc::new(AtomicUsize::new(0));
    // let mut handles = vec![];

    // for string_to_build in strings_to_build {
    //     let segments = segments
    //         .clone()
    //         .iter()
    //         .map(|segment| segment.clone())
    //         .filter(|segment| string_to_build.contains(segment))
    //         .collect::<Vec<String>>();

    //     let counter = Arc::clone(&total_possible);
    //     let handle = thread::spawn(move || {
    //         if can_be_made_from_segments(&segments, string_to_build.as_str()) {
    //             counter.fetch_add(
    //                 count_number_of_solutions(&segments, string_to_build.as_str()),
    //                 std::sync::atomic::Ordering::SeqCst,
    //             );
    //         }
    //     });
    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // println!(
    //     "total possible solutions: {}",
    //     total_possible.load(std::sync::atomic::Ordering::SeqCst)
    // )
}

#[test]
fn is_not_possible() {
    let segments = vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"]
        .iter()
        .map(|value| value.to_string())
        .collect();

    let string_to_build = "ubwu";

    let is_possible = can_be_made_from_segments(&segments, string_to_build);

    assert_eq!(is_possible, false);
}

#[test]
fn is_possible() {
    let segments = vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"]
        .iter()
        .map(|value| value.to_string())
        .collect();

    let string_to_build = "brwrr";

    let is_possible = can_be_made_from_segments(&segments, string_to_build);

    assert_eq!(is_possible, true);
}

#[test]
fn is_possible_2() {
    let segments = vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"]
        .iter()
        .map(|value| value.to_string())
        .collect();

    let string_to_build = "bwurrg";

    let is_possible = can_be_made_from_segments(&segments, string_to_build);

    assert_eq!(is_possible, true);
}

#[test]
fn example() {
    let segments = vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"]
        .iter()
        .map(|value| value.to_string())
        .collect();

    let strings_to_build = vec![
        "brwrr", "bggr", "gbbr", "rrbgbr", "ubwu", "bwurrg", "brgr", "bbrgwb",
    ];

    let mut total_possible = 0;
    for string_to_build in strings_to_build {
        if can_be_made_from_segments(&segments, string_to_build) {
            total_possible += 1;
        }
    }

    assert_eq!(total_possible, 6);
}

#[test]
fn example_part_2() {
    let segments = vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"]
        .iter()
        .map(|value| value.to_string())
        .collect();

    let strings_to_build = vec![
        "brwrr", "bggr", "gbbr", "rrbgbr", "ubwu", "bwurrg", "brgr", "bbrgwb",
    ];

    let mut total_solutions = 0;
    for string_to_build in strings_to_build {
        total_solutions += count_number_of_solutions(&segments, string_to_build);
    }

    assert_eq!(total_solutions, 16);
}

#[test]
fn example_part_2_rewrite() {
    let segments = vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"]
        .iter()
        .map(|value| value.to_string())
        .collect();

    let strings_to_build = vec![
        "brwrr", "bggr", "gbbr", "rrbgbr", "ubwu", "bwurrg", "brgr", "bbrgwb",
    ];

    let total_solutions = count_number_of_solutions_rewrite(&segments, strings_to_build);

    assert_eq!(total_solutions, 16);
}
