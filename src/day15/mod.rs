use std::fs;

use regex::{Match, Regex};

#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Clone, Copy)]
enum TileType {
    Wall,
    Empty,
    Box,
    Robot,
}

#[derive(PartialEq, Clone, Copy)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Clone, Copy)]
struct WarehouseTile {
    coordinate: Coordinate,
    contents: TileType,
}

fn read_input() -> (Vec<WarehouseTile>, Vec<Direction>) {
    let input = fs::read_to_string("src/day15/input.txt").expect("No input for day 15");

    let map_regex = Regex::new(r"[#.O@]+").unwrap();
    let moves_regex = Regex::new(r"[<>^v]").unwrap();

    let moves = moves_regex
        .find_iter(&input)
        .map(|direction| map_char_to_direction(direction.as_str()))
        .collect::<Vec<Direction>>();

    let warehouse_floor_matches = map_regex.find_iter(&input).collect::<Vec<Match>>();

    let mut warehouse_floor = vec![];

    for y in 0..warehouse_floor_matches.len() {
        let line = warehouse_floor_matches[y]
            .as_str()
            .chars()
            .collect::<Vec<char>>();
        for x in 0..line.len() {
            let char = line[x];
            warehouse_floor.push(WarehouseTile {
                coordinate: Coordinate { x, y },
                contents: map_char_to_tile_type(char),
            });
        }
    }

    return (warehouse_floor, moves);
}

fn map_char_to_direction(char: &str) -> Direction {
    match char {
        "<" => Direction::Left,
        "^" => Direction::Up,
        ">" => Direction::Right,
        "v" => Direction::Down,
        _ => panic!("Invalid Direction Char"),
    }
}

fn map_char_to_tile_type(char: char) -> TileType {
    match char {
        '#' => TileType::Wall,
        '.' => TileType::Empty,
        'O' => TileType::Box,
        '@' => TileType::Robot,
        _ => panic!("Invalid map character"),
    }
}

fn get_next_coordinate_in_direction(coordinate: Coordinate, direction: &Direction) -> Coordinate {
    match direction {
        Direction::Up => Coordinate {
            x: coordinate.x,
            y: coordinate.y - 1,
        },
        Direction::Down => Coordinate {
            x: coordinate.x,
            y: coordinate.y + 1,
        },
        Direction::Left => Coordinate {
            x: coordinate.x - 1,
            y: coordinate.y,
        },
        Direction::Right => Coordinate {
            x: coordinate.x + 1,
            y: coordinate.y,
        },
    }
}

fn move_robot_in_direction(floor: &mut Vec<WarehouseTile>, direction: &Direction) {
    let floor_before_move = floor.clone();
    // println!("Moving {:?}", direction);
    let robot_tile = floor_before_move
        .iter()
        .find(|tile| tile.contents == TileType::Robot)
        .unwrap();

    let mut current_tile = robot_tile;

    let mut new_tile_contents = vec![];

    loop {
        let tile_on_current_coordinate = floor
            .iter()
            .find(|tile| tile.coordinate == current_tile.coordinate)
            .unwrap();

        if tile_on_current_coordinate.contents == TileType::Wall {
            return;
        }
        match tile_on_current_coordinate.contents {
            TileType::Wall => return,
            TileType::Empty => {
                new_tile_contents.push(current_tile);
                for tile_to_move in new_tile_contents {
                    let relevant_tile =
                        floor.iter_mut().find(|tile| *tile == tile_to_move).unwrap();

                    if relevant_tile.contents == TileType::Empty {
                        relevant_tile.coordinate = robot_tile.coordinate;
                    } else {
                        relevant_tile.coordinate =
                            get_next_coordinate_in_direction(relevant_tile.coordinate, direction)
                    }
                }
                return;
            }
            _ => {
                new_tile_contents.push(current_tile);
                current_tile = floor_before_move
                    .iter()
                    .find(|tile| {
                        tile.coordinate
                            == get_next_coordinate_in_direction(current_tile.coordinate, direction)
                    })
                    .unwrap()
            }
        }
    }
}

fn print_board(floor: &Vec<WarehouseTile>) {
    let mut board_string = String::new();

    let mut y = 0;
    loop {
        let mut x = 0;
        loop {
            let current_coordinate = Coordinate { x, y };
            if let Some(tile_at_coordinate) = floor
                .iter()
                .find(|tile| tile.coordinate == current_coordinate)
            {
                let char_to_add = match tile_at_coordinate.contents {
                    TileType::Wall => '#',
                    TileType::Empty => '.',
                    TileType::Box => 'O',
                    TileType::Robot => '@',
                };
                board_string.push(char_to_add);
                x += 1;
            } else {
                break;
            }
        }
        y += 1;
        let current_coordinate = Coordinate { x: 0, y };
        if let None = floor
            .iter()
            .find(|tile| tile.coordinate == current_coordinate)
        {
            break;
        }
        board_string.push('\n');
    }

    println!("{board_string}");
}

fn calculate_gps_coordinate(coordinate: Coordinate) -> usize {
    return coordinate.y * 100 + coordinate.x;
}

fn calculate_total_gps_score(floor: &Vec<WarehouseTile>) {
    let boxes = floor
        .iter()
        .filter(|tile| tile.contents == TileType::Box)
        .map(|tile| calculate_gps_coordinate(tile.coordinate))
        .sum::<usize>();

    println!("Total gps score: {}", boxes)
}

pub fn day_15_part_1() {
    let (mut floor, moves) = read_input();

    print_board(&floor);

    for direction in moves {
        move_robot_in_direction(&mut floor, &direction);
        // print_board(&floor);
    }

    print_board(&floor);
    calculate_total_gps_score(&floor);
}
