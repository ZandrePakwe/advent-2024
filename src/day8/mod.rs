use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::Index,
};

fn read_input() -> Vec<Vec<char>> {
    let text = fs::read_to_string("src/day8/input.txt").expect("input file not found");

    let vector_2d: Vec<Vec<char>> = text
        .split("\n")
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    return vector_2d;
}

pub fn day_8_part_1() {
    let input = read_input();

    let antennas = get_antenna_groupings(&input);

    let antinodes = calculate_antinode_coordinates(antennas, input.len(), input[1].len());

    // draw_antinodes(&input, antinodes.clone());

    println!("Total of {} antinodes", antinodes.len())
}

#[derive(Clone, Copy, PartialEq, Hash, Eq, Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

fn get_antenna_groupings(input: &Vec<Vec<char>>) -> HashMap<char, Vec<Coordinate>> {
    let mut antennas = HashMap::<char, Vec<Coordinate>>::new();

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            let char = input[y][x];

            if char != '.' {
                antennas
                    .entry(char)
                    .or_insert(vec![])
                    .push(Coordinate { x, y });
            }
        }
    }

    return antennas;
}

fn calculate_antinode_coordinates(
    antenna_grouping: HashMap<char, Vec<Coordinate>>,
    x_max: usize,
    y_max: usize,
) -> HashSet<Coordinate> {
    let mut antinode_coordinates = HashSet::<Coordinate>::new();

    for (_, coordinates) in antenna_grouping {
        let mut new_coordinates = coordinates.clone();
        for _ in 0..coordinates.len() {
            let current_coordinate = new_coordinates.remove(0);

            let remaining_coordinates = new_coordinates.clone();

            for comparison_coordinate in remaining_coordinates {
                let (antinode_1, antinode_2) =
                    calculate_antinodes(&current_coordinate, &comparison_coordinate, x_max, y_max);
                if let Some(antinode) = antinode_1 {
                    antinode_coordinates.insert(antinode);
                }
                if let Some(antinode) = antinode_2 {
                    antinode_coordinates.insert(antinode);
                }
            }
        }
    }

    return antinode_coordinates;
}

fn calculate_antinodes(
    coordinate: &Coordinate,
    comparison_coordinate: &Coordinate,
    x_max: usize,
    y_max: usize,
) -> (Option<Coordinate>, Option<Coordinate>) {
    let delta_x = comparison_coordinate.x as i32 - coordinate.x as i32;
    let delta_y = comparison_coordinate.y as i32 - coordinate.y as i32;

    let x_1 = coordinate.x as i32 - delta_x;
    let x_2 = comparison_coordinate.x as i32 + delta_x;
    let y_1 = coordinate.y as i32 - delta_y;
    let y_2 = comparison_coordinate.y as i32 + delta_y;
    let mut antinode_1 = None;
    let mut antinode_2 = None;
    if x_1 >= 0 && x_1 < x_max as i32 && y_1 >= 0 && y_1 < y_max as i32 {
        antinode_1 = Some(Coordinate {
            x: x_1 as usize,
            y: y_1 as usize,
        });
    }
    if x_2 >= 0 && x_2 < x_max as i32 && y_2 >= 0 && y_2 < y_max as i32 {
        antinode_2 = Some(Coordinate {
            x: x_2 as usize,
            y: y_2 as usize,
        });
    }

    return (antinode_1, antinode_2);
}

fn draw_antinodes(input: &Vec<Vec<char>>, antinodes: HashSet<Coordinate>) {
    let mut input = input.clone();
    for coordinate in antinodes {
        input[coordinate.y][coordinate.x] = '#'
    }

    print_board(&input);
}

fn print_board(input: &Vec<Vec<char>>) {
    let mut final_string = "".to_string();
    for column in input {
        for character in column {
            final_string += &character.to_string();
        }
        final_string += "\n";
    }

    println!("{final_string}");
}
