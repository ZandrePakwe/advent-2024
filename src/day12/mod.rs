use std::{
    collections::{HashMap, HashSet},
    fs,
};

use strum::{EnumIter, IntoEnumIterator};

fn read_input() -> Vec<Vec<char>> {
    let input = fs::read_to_string("src/day12/input.txt").expect("Day 12 input not present");

    let vector_2d: Vec<Vec<char>> = input
        .split("\n")
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    return vector_2d;
}

pub fn day_12_part_1() {
    let input = read_input();
    let mut unexplored_coordinates = get_coordinates(&input);

    let mut total_price = 0;

    loop {
        if unexplored_coordinates.is_empty() {
            break;
        }

        if let Some(coordinate) = unexplored_coordinates.iter().next() {
            let (coordinates, circumference) = get_block_at_coordinate(&input, coordinate);
            let area_of_block = coordinates.len();

            for coordinate in coordinates {
                unexplored_coordinates.remove(&coordinate);
            }
            let price = area_of_block * circumference;

            total_price += price;
        }
    }

    println!("Total Price of Fences with circumference: {total_price}");
}

pub fn day_12_part_2() {
    let input = read_input();
    let mut unexplored_coordinates = get_coordinates(&input);

    let mut total_price = 0;

    loop {
        if unexplored_coordinates.is_empty() {
            break;
        }

        if let Some(coordinate) = unexplored_coordinates.iter().next() {
            let (coordinates, _) = get_block_at_coordinate(&input, coordinate);
            let area_of_block = coordinates.len();
            let edges = count_sides(&coordinates);

            for coordinate in coordinates {
                unexplored_coordinates.remove(&coordinate);
            }

            let price = area_of_block * edges;

            total_price += price;
        }
    }

    println!("Total Price of Fences with Sides: {total_price}");
}

#[derive(PartialEq, EnumIter, Hash, Clone, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct CoordinateEdge {
    coordinate: Coordinate,
    direction: Direction,
}

fn get_block_at_coordinate(
    input: &Vec<Vec<char>>,
    coordinate: &Coordinate,
) -> (HashSet<Coordinate>, usize) {
    let mut coordinates_in_block = HashSet::new();
    let mut coordinates_to_try = HashSet::new();
    let mut circumference = 0;

    coordinates_in_block.insert(coordinate.clone());
    coordinates_to_try.insert(coordinate.clone());

    loop {
        for coordinate in coordinates_to_try.clone() {
            coordinates_to_try.remove(&coordinate);
            let (new_coordinates, additional_circumference) =
                get_adjacent_of_type_at_coordinate(input, &coordinate);
            circumference += additional_circumference;
            for new_coordinate in new_coordinates {
                if let None = coordinates_in_block.get(&new_coordinate) {
                    coordinates_in_block.insert(new_coordinate.clone());
                    coordinates_to_try.insert(new_coordinate);
                }
            }
        }
        if coordinates_to_try.len() == 0 {
            return (coordinates_in_block, circumference);
        }
    }
}

fn get_adjacent_of_type_at_coordinate(
    input: &Vec<Vec<char>>,
    coordinate: &Coordinate,
) -> (HashSet<Coordinate>, usize) {
    let mut coordinates = HashSet::new();
    let mut circumference = 0;
    let plant_type = input[coordinate.y][coordinate.x];
    coordinates.insert(coordinate.clone());

    for direction in Direction::iter() {
        if let Some(coordinate) = get_bound_coordinate_in_direction(&coordinate, direction, input) {
            let adjacent_plant_type = input[coordinate.y][coordinate.x];
            if adjacent_plant_type == plant_type {
                coordinates.insert(coordinate);
                continue;
            }
            circumference += 1;
            continue;
        }
        circumference += 1;
    }
    return (coordinates, circumference);
}

fn get_bound_coordinate_in_direction(
    coordinate: &Coordinate,
    direction: Direction,
    input: &Vec<Vec<char>>,
) -> Option<Coordinate> {
    match direction {
        Direction::Up => {
            if coordinate.y == 0 {
                return None;
            }
            return Some(Coordinate {
                x: coordinate.x,
                y: coordinate.y - 1,
            });
        }
        Direction::Down => {
            if coordinate.y == input.len() - 1 {
                return None;
            }
            return Some(Coordinate {
                x: coordinate.x,
                y: coordinate.y + 1,
            });
        }
        Direction::Right => {
            if coordinate.x == input[coordinate.y].len() - 1 {
                return None;
            }
            return Some(Coordinate {
                x: coordinate.x + 1,
                y: coordinate.y,
            });
        }
        Direction::Left => {
            if coordinate.x == 0 {
                return None;
            }
            return Some(Coordinate {
                x: coordinate.x - 1,
                y: coordinate.y,
            });
        }
    }
}

fn get_coordinate_in_direction(
    coordinate: &Coordinate,
    direction: Direction,
    block: &HashSet<Coordinate>,
) -> Option<Coordinate> {
    match direction {
        Direction::Up => {
            if coordinate.y == 0 {
                return None;
            }
            return block
                .get(&Coordinate {
                    x: coordinate.x,
                    y: coordinate.y - 1,
                })
                .cloned();
        }
        Direction::Down => {
            return block
                .get(&Coordinate {
                    x: coordinate.x,
                    y: coordinate.y + 1,
                })
                .cloned();
        }
        Direction::Right => {
            return block
                .get(&Coordinate {
                    x: coordinate.x + 1,
                    y: coordinate.y,
                })
                .cloned();
        }
        Direction::Left => {
            if coordinate.x == 0 {
                return None;
            }
            return block
                .get(&Coordinate {
                    x: coordinate.x - 1,
                    y: coordinate.y,
                })
                .cloned();
        }
    }
}

fn count_sides(coordinates: &HashSet<Coordinate>) -> usize {
    let mut edge_coordinates = vec![];
    for coordinate in coordinates {
        for direction in Direction::iter() {
            if let None = get_coordinate_in_direction(coordinate, direction.clone(), coordinates) {
                edge_coordinates.push(CoordinateEdge {
                    coordinate: coordinate.clone(),
                    direction: direction,
                });
            }
        }
    }

    let mut edges = 0;

    for direction in Direction::iter() {
        let edges_in_direction = edge_coordinates
            .iter()
            .filter(|edge| edge.direction == direction)
            .map(|edge| edge.coordinate)
            .collect::<Vec<Coordinate>>();

        let mut perpendicular_lines: HashMap<usize, Vec<Coordinate>> = HashMap::new();

        for edge in edges_in_direction {
            let key = if direction == Direction::Up || direction == Direction::Down {
                edge.y
            } else {
                edge.x
            };
            perpendicular_lines.entry(key).or_insert(vec![]).push(edge);
        }

        for (_, mut coordinates) in perpendicular_lines {
            coordinates.sort_by(|coordinate_a, coordinate_b| {
                if direction == Direction::Up || direction == Direction::Down {
                    return coordinate_a.x.cmp(&coordinate_b.x);
                }
                return coordinate_a.y.cmp(&coordinate_b.y);
            });

            let mut last_index = usize::max_value();
            for coordinate in coordinates {
                let new_index = if direction == Direction::Up || direction == Direction::Down {
                    coordinate.x
                } else {
                    coordinate.y
                };

                if new_index.abs_diff(last_index) > 1 {
                    edges += 1;
                }

                last_index = new_index;
            }
        }
    }

    return edges;
}

fn get_coordinates(input: &Vec<Vec<char>>) -> HashSet<Coordinate> {
    let mut response = HashSet::new();
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            response.insert(Coordinate { x, y });
        }
    }
    return response;
}
