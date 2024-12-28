use std::{
    collections::{HashMap, HashSet},
    fs,
};

use strum::{EnumIter, IntoEnumIterator};

fn read_input() -> Vec<Vec<u32>> {
    let text = fs::read_to_string("src/day10/input.txt").expect("input file not found");

    let vector_2d: Vec<Vec<u32>> = text
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    return vector_2d;
}

pub fn day_10_part_1() {
    let input = read_input();
    let starting_points = find_all_start_locations(&input);
    print_board(&input);

    let mut total = 0;
    for point in starting_points {
        let paths_to_summit = find_next_viable_steps_with_directions(&input, point);

        total += paths_to_summit.len();
    }
    println!("total: {total}")
}

pub fn day_10_part_2() {
    let input = read_input();
    let starting_points = find_all_start_locations(&input);

    let mut total = 0;
    for point in starting_points {
        let paths_to_summit = find_next_viable_steps_with_directions(&input, point);

        for (_, path_count) in paths_to_summit {
            total += path_count
        }
    }
    println!("total: {total}")
}

fn find_all_start_locations(input: &Vec<Vec<u32>>) -> Vec<Coordinate> {
    let mut coordinates = vec![];
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            let value = input[y][x];
            if value == 0 {
                coordinates.push(Coordinate { x, y });
            }
        }
    }

    return coordinates;
}

fn find_next_viable_steps_with_directions(
    input: &Vec<Vec<u32>>,
    coordinate: Coordinate,
) -> HashMap<Coordinate, usize> {
    let value = input[coordinate.y][coordinate.x];

    let mut summits_reachable = HashMap::new();

    for direction in Direction::iter() {
        if let Some((value_in_direction, coordinate)) =
            value_in_direction(input, coordinate, direction.clone())
        {
            if (value + 1) == value_in_direction {
                if value_in_direction == 9 {
                    *summits_reachable.entry(coordinate).or_insert(0) += 1;
                }
                let summits = find_next_viable_steps_with_directions(input, coordinate);
                for (coordinate, count) in summits {
                    *summits_reachable.entry(coordinate).or_insert(0) += count;
                }
            }
        }
    }

    return summits_reachable;
}

fn value_in_direction(
    input: &Vec<Vec<u32>>,
    coordinate: Coordinate,
    direction: Direction,
) -> Option<(u32, Coordinate)> {
    match direction {
        Direction::Up => {
            if coordinate.y == 0 {
                return None;
            }
            let coordinate = Coordinate {
                x: coordinate.x,
                y: coordinate.y - 1,
            };
            return Some((input[coordinate.y][coordinate.x], coordinate));
        }
        Direction::Down => {
            if coordinate.y == input.len() - 1 {
                return None;
            }
            let coordinate = Coordinate {
                x: coordinate.x,
                y: coordinate.y + 1,
            };
            return Some((input[coordinate.y][coordinate.x], coordinate));
        }
        Direction::Left => {
            if coordinate.x == 0 {
                return None;
            }
            let coordinate = Coordinate {
                x: coordinate.x - 1,
                y: coordinate.y,
            };
            return Some((input[coordinate.y][coordinate.x], coordinate));
        }
        Direction::Right => {
            if coordinate.x == input.len() - 1 {
                return None;
            }
            let coordinate = Coordinate {
                x: coordinate.x + 1,
                y: coordinate.y,
            };
            return Some((input[coordinate.y][coordinate.x], coordinate));
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(EnumIter, Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn print_board(input: &Vec<Vec<u32>>) {
    let mut final_string = "".to_string();
    for column in input {
        for character in column {
            final_string += &character.to_string();
        }
        final_string += "\n";
    }

    println!("{final_string}");
}
