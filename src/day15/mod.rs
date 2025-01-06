use std::{collections::HashSet, fs};

use peroxide::fuga::max;
use regex::{Match, Regex};

#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Clone, Copy, Eq, Hash)]
enum TileType {
    Wall,
    Empty,
    Box,
    BoxLeft,
    BoxRight,
    Robot,
}

#[derive(PartialEq, Clone, Copy, Eq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Clone, Copy, Eq, Hash)]
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

fn double_width_on_input(input: Vec<WarehouseTile>) -> Vec<WarehouseTile> {
    let mut new_floor = vec![];

    for tile in input {
        let mut new_tile_1 = tile.clone();
        new_tile_1.coordinate.x *= 2;
        let mut new_tile_2 = new_tile_1.clone();
        new_tile_2.coordinate.x += 1;

        match tile.contents {
            TileType::Box => {
                new_tile_1.contents = TileType::BoxLeft;
                new_tile_2.contents = TileType::BoxRight;
            }
            TileType::Robot => new_tile_2.contents = TileType::Empty,
            _ => {}
        }

        new_floor.push(new_tile_1);
        new_floor.push(new_tile_2);
    }

    return new_floor;
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

fn move_robot_in_direction_wide(floor: &mut Vec<WarehouseTile>, direction: &Direction) {
    let floor_before_move = floor.clone();
    let robot_tile = floor_before_move
        .iter()
        .find(|tile| tile.contents == TileType::Robot)
        .unwrap();

    let next_tile_to_investigate = floor_before_move
        .iter()
        .find(|tile| {
            tile.coordinate == get_next_coordinate_in_direction(robot_tile.coordinate, direction)
        })
        .unwrap();

    let mut tiles_to_move = HashSet::new();

    tiles_to_move.insert(*robot_tile);

    let mut tiles_to_investigate = vec![next_tile_to_investigate];

    loop {
        let mut next_tiles_to_investigate = vec![];
        for next_tile in tiles_to_investigate.clone() {
            match next_tile.contents {
                TileType::Wall => return,
                TileType::Empty => {
                    tiles_to_move.insert(*next_tile);
                }
                TileType::BoxLeft => {
                    let other_half_of_box = floor_before_move
                        .iter()
                        .find(|tile| {
                            tile.coordinate
                                == get_next_coordinate_in_direction(
                                    next_tile.coordinate,
                                    &Direction::Right,
                                )
                        })
                        .unwrap();

                    let next_tile_1 = floor_before_move
                        .iter()
                        .find(|tile| {
                            tile.coordinate
                                == get_next_coordinate_in_direction(next_tile.coordinate, direction)
                        })
                        .unwrap();

                    let next_tile_2 = floor_before_move
                        .iter()
                        .find(|tile| {
                            tile.coordinate
                                == get_next_coordinate_in_direction(
                                    other_half_of_box.coordinate,
                                    direction,
                                )
                        })
                        .unwrap();

                    tiles_to_move.insert(*next_tile);
                    tiles_to_move.insert(*other_half_of_box);
                    next_tiles_to_investigate.push(next_tile_1);
                    next_tiles_to_investigate.push(next_tile_2);
                }
                TileType::BoxRight => {
                    let other_half_of_box = floor_before_move
                        .iter()
                        .find(|tile| {
                            tile.coordinate
                                == get_next_coordinate_in_direction(
                                    next_tile.coordinate,
                                    &Direction::Left,
                                )
                        })
                        .unwrap();

                    let next_tile_1 = floor_before_move
                        .iter()
                        .find(|tile| {
                            tile.coordinate
                                == get_next_coordinate_in_direction(next_tile.coordinate, direction)
                        })
                        .unwrap();

                    let next_tile_2 = floor_before_move
                        .iter()
                        .find(|tile| {
                            tile.coordinate
                                == get_next_coordinate_in_direction(
                                    other_half_of_box.coordinate,
                                    direction,
                                )
                        })
                        .unwrap();

                    tiles_to_move.insert(*next_tile);
                    tiles_to_move.insert(*other_half_of_box);
                    next_tiles_to_investigate.push(next_tile_1);
                    next_tiles_to_investigate.push(next_tile_2);
                }
                _ => todo!(),
            }
        }
        if next_tiles_to_investigate.len() == 0 {
            break;
        }

        tiles_to_investigate = next_tiles_to_investigate
            .iter()
            .filter(|tile_to_investigate| tiles_to_move.get(&tile_to_investigate) == None)
            .map(|tile| *tile)
            .collect();
    }

    move_tiles(floor, tiles_to_move, direction);
}

fn move_tiles(
    floor: &mut Vec<WarehouseTile>,
    tiles: HashSet<WarehouseTile>,
    direction: &Direction,
) {
    for tile in tiles.clone() {
        let tile = floor
            .iter_mut()
            .find(|floor_tile| **floor_tile == tile)
            .unwrap();

        let last_in_slice_in_direction =
            find_last_coordinate_in_slice_in_direction(tile, &tiles, direction);

        if tile.coordinate == last_in_slice_in_direction {
            let first_coordinate =
                find_first_coordinate_in_slice_in_direction(tile, &tiles, direction);
            tile.coordinate = first_coordinate;
        } else {
            tile.coordinate = get_next_coordinate_in_direction(tile.coordinate, direction)
        }
    }
}

fn find_first_coordinate_in_slice_in_direction(
    tile: &WarehouseTile,
    tiles: &HashSet<WarehouseTile>,
    direction: &Direction,
) -> Coordinate {
    match direction {
        Direction::Up => {
            let mut current_tile = tile;
            while let Some(previous_tile) = tiles.iter().find(|floor_tile| {
                floor_tile.coordinate.y == current_tile.coordinate.y + 1
                    && floor_tile.coordinate.x == current_tile.coordinate.x
            }) {
                current_tile = previous_tile;
            }

            return current_tile.coordinate;
        }
        Direction::Down => {
            let mut current_tile = tile;
            while let Some(previous_tile) = tiles.iter().find(|floor_tile| {
                floor_tile.coordinate.y == current_tile.coordinate.y - 1
                    && floor_tile.coordinate.x == current_tile.coordinate.x
            }) {
                current_tile = previous_tile;
            }

            return current_tile.coordinate;
        }
        Direction::Left => {
            let mut current_tile = tile;
            while let Some(previous_tile) = tiles.iter().find(|floor_tile| {
                floor_tile.coordinate.x == current_tile.coordinate.x + 1
                    && floor_tile.coordinate.y == current_tile.coordinate.y
            }) {
                current_tile = previous_tile;
            }

            return current_tile.coordinate;
        }
        Direction::Right => {
            let mut current_tile = tile;
            while let Some(previous_tile) = tiles.iter().find(|floor_tile| {
                floor_tile.coordinate.x == current_tile.coordinate.x - 1
                    && floor_tile.coordinate.y == current_tile.coordinate.y
            }) {
                current_tile = previous_tile;
            }

            return current_tile.coordinate;
        }
    }
}

fn find_last_coordinate_in_slice_in_direction(
    tile: &WarehouseTile,
    tiles: &HashSet<WarehouseTile>,
    direction: &Direction,
) -> Coordinate {
    match direction {
        Direction::Up => {
            let mut current_tile = tile;
            while let Some(previous_tile) = tiles.iter().find(|floor_tile| {
                floor_tile.coordinate.y == current_tile.coordinate.y - 1
                    && floor_tile.coordinate.x == current_tile.coordinate.x
            }) {
                current_tile = previous_tile;
            }

            return current_tile.coordinate;
        }
        Direction::Down => {
            let mut current_tile = tile;
            while let Some(previous_tile) = tiles.iter().find(|floor_tile| {
                floor_tile.coordinate.y == current_tile.coordinate.y + 1
                    && floor_tile.coordinate.x == current_tile.coordinate.x
            }) {
                current_tile = previous_tile;
            }

            return current_tile.coordinate;
        }
        Direction::Left => {
            let mut current_tile = tile;
            while let Some(previous_tile) = tiles.iter().find(|floor_tile| {
                floor_tile.coordinate.x == current_tile.coordinate.x - 1
                    && floor_tile.coordinate.y == current_tile.coordinate.y
            }) {
                current_tile = previous_tile;
            }

            return current_tile.coordinate;
        }
        Direction::Right => {
            let mut current_tile = tile;
            while let Some(previous_tile) = tiles.iter().find(|floor_tile| {
                floor_tile.coordinate.x == current_tile.coordinate.x + 1
                    && floor_tile.coordinate.y == current_tile.coordinate.y
            }) {
                current_tile = previous_tile;
            }

            return current_tile.coordinate;
        }
    }
}

fn print_board(floor: &Vec<WarehouseTile>) {
    let mut board_string = String::new();
    let mut max_x = 0;
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
                    TileType::BoxLeft => '[',
                    TileType::BoxRight => ']',
                };
                board_string.push(char_to_add);
                x += 1;
                max_x = max(vec![x, max_x]);
            } else {
                if x < max_x {
                    panic!("something terrible happened")
                }
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
        .filter(|tile| tile.contents == TileType::Box || tile.contents == TileType::BoxLeft)
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

pub fn day_15_part_2() {
    let (mut floor, moves) = read_input();

    floor = double_width_on_input(floor);

    print_board(&floor);

    for direction in moves {
        move_robot_in_direction_wide(&mut floor, &direction);
        // print_board(&floor);
    }

    print_board(&floor);
    calculate_total_gps_score(&floor);
}
