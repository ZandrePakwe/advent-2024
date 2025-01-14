use std::{
    collections::{HashMap, HashSet},
    fs, usize,
};

use strum::{EnumIter, IntoEnumIterator};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate {
    x: isize,
    y: isize,
}

struct Racetrack {
    tiles: HashMap<Coordinate, MapTile>,
    start: Coordinate,
    end: Coordinate,
}

#[derive(Clone, Copy)]
struct MapTile {
    tile: Tile,
    distance: usize,
}

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    Track,
    Wall,
    Start,
    End,
}

#[derive(EnumIter)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn convert_string_to_racetrack(input: String) -> Racetrack {
    let mut racetrack = HashMap::new();

    let mut start = Coordinate { x: 0, y: 0 };
    let mut end = Coordinate { x: 0, y: 0 };

    for y in 0..input.lines().count() {
        for x in 0..input.lines().nth(y).unwrap().len() {
            let char = input.lines().nth(y).unwrap().chars().nth(x).unwrap();

            let x = x as isize;
            let y = y as isize;
            let coordinate = Coordinate { x, y };

            let tile = match char {
                '.' => Tile::Track,
                'S' => {
                    start = coordinate;
                    Tile::Start
                }
                'E' => {
                    end = coordinate;
                    Tile::End
                }
                _ => Tile::Wall,
            };

            let map_tile = MapTile {
                tile,
                distance: usize::MAX,
            };

            racetrack.entry(coordinate).insert_entry(map_tile);
        }
    }

    return Racetrack {
        tiles: racetrack,
        start,
        end,
    };
}

fn read_input() -> Racetrack {
    let input = fs::read_to_string("src/day20/input.txt").expect("Day 20 input missing");

    return convert_string_to_racetrack(input);
}

fn get_coordinate_in_direction(coordinate: &Coordinate, direction: Direction) -> Coordinate {
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

fn get_next_coordinates(
    racetrack: &HashMap<Coordinate, MapTile>,
    coordinate: &Coordinate,
) -> Vec<Coordinate> {
    let mut result = vec![];

    for direction in Direction::iter() {
        let coordinate_to_check = get_coordinate_in_direction(coordinate, direction);

        if let Some(entry) = racetrack.get(&coordinate_to_check) {
            if entry.tile != Tile::Wall && entry.distance == usize::MAX {
                result.push(coordinate_to_check);
            }
        }
    }

    return result;
}

fn get_next_coordinates_in_range(
    racetrack: &HashMap<Coordinate, MapTile>,
    coordinate: &Coordinate,
    size: usize,
) -> HashSet<Coordinate> {
    let mut result = HashSet::new();
    let size = size as isize;

    let min = size * -1;
    let max = size;

    for x in min..=max {
        let min = min + x.abs();
        let max = min * -1;
        for y in min..=max {
            let new_coordinate = Coordinate {
                x: coordinate.x + x,
                y: coordinate.y + y,
            };
            if let Some(entry) = racetrack.get(&new_coordinate) {
                if entry.tile != Tile::Wall {
                    result.insert(new_coordinate);
                }
            }
        }
    }

    return result;
}

fn get_distance_of_track(racetrack: &mut Racetrack) -> usize {
    let start = racetrack.start;

    let start_tile = racetrack.tiles.get_mut(&start).unwrap();

    start_tile.distance = 0;

    let mut next_tiles = vec![start];
    let mut last_distance = start_tile.distance;

    while let Some(coordinate) = next_tiles.pop() {
        let next_coordinates = get_next_coordinates(&racetrack.tiles, &coordinate);
        last_distance += 1;
        for coordinate in next_coordinates {
            next_tiles.push(coordinate);
            let tile = racetrack.tiles.get_mut(&coordinate).unwrap();
            tile.distance = last_distance;
        }
    }

    let end_tile = racetrack.tiles.get_mut(&racetrack.end).unwrap();

    return end_tile.distance;
}

fn find_shortcuts(racetrack: &mut Racetrack, shortcut_length: usize) -> HashMap<usize, usize> {
    let mut shortcuts = HashMap::new();

    for (coordinate, tile) in racetrack.tiles.clone() {
        if tile.tile != Tile::Wall {
            let adjacent_tiles = get_next_coordinates_in_range(&racetrack.tiles, &coordinate, shortcut_length);

            for tile_to_compare in adjacent_tiles {
                let delta_to_tile = coordinate.x.abs_diff(tile_to_compare.x)
                    + coordinate.y.abs_diff(tile_to_compare.y);
                let new_distance_to_tile_after_shortcut = tile.distance + delta_to_tile;

                let tile_after_shortcut = racetrack.tiles.get(&tile_to_compare).unwrap();

                let time_saved_with_shortcut = tile_after_shortcut.distance as isize
                    - new_distance_to_tile_after_shortcut as isize;

                if time_saved_with_shortcut > 0 {
                    *shortcuts
                        .entry(time_saved_with_shortcut as usize)
                        .or_insert(0) += 1
                }
            }
        }
    }

    return shortcuts;
}

pub fn count_number_of_shortcuts_saving_at_least_100(shortcuts: HashMap<usize, usize>) -> usize {
    let mut total = 0;
    for (time_saved, number_of_occurences) in shortcuts {
        if time_saved >= 100 {
            total += number_of_occurences;
        }
    }

    return total;
}

pub fn day_20_part_1() {
    let mut racetrack = read_input();

    get_distance_of_track(&mut racetrack);

    let shortcuts = find_shortcuts(&mut racetrack, 2);

    let shortcuts_saving_100_or_more = count_number_of_shortcuts_saving_at_least_100(shortcuts);

    println!(
        "There are {} shortcuts savinf 100 picoseconds or more",
        shortcuts_saving_100_or_more
    );
}

pub fn day_20_part_2() {
    let mut racetrack = read_input();

    get_distance_of_track(&mut racetrack);

    let shortcuts = find_shortcuts(&mut racetrack, 20);

    let shortcuts_saving_100_or_more = count_number_of_shortcuts_saving_at_least_100(shortcuts);

    println!(
        "There are {} shortcuts savinf 100 picoseconds or more",
        shortcuts_saving_100_or_more
    );
}

#[test]
fn get_example_track_distance() {
    let race_track = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"
        .to_string();
    let mut racetrack = convert_string_to_racetrack(race_track);

    let track_length = get_distance_of_track(&mut racetrack);

    assert_eq!(track_length, 84)
}

#[test]
fn get_example_track_shortcuts() {
    let race_track = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"
        .to_string();
    let mut racetrack = convert_string_to_racetrack(race_track);

    get_distance_of_track(&mut racetrack);

    let shortcuts = find_shortcuts(&mut racetrack, 2);

    let number_saving_2 = *shortcuts.get(&2).unwrap_or(&0);
    let number_saving_4 = *shortcuts.get(&4).unwrap_or(&0);
    let number_saving_6 = *shortcuts.get(&6).unwrap_or(&0);
    let number_saving_8 = *shortcuts.get(&8).unwrap_or(&0);
    let number_saving_10 = *shortcuts.get(&10).unwrap_or(&0);
    let number_saving_12 = *shortcuts.get(&12).unwrap_or(&0);
    let number_saving_20 = *shortcuts.get(&20).unwrap_or(&0);
    let number_saving_36 = *shortcuts.get(&36).unwrap_or(&0);
    let number_saving_38 = *shortcuts.get(&38).unwrap_or(&0);
    let number_saving_40 = *shortcuts.get(&40).unwrap_or(&0);
    let number_saving_64 = *shortcuts.get(&64).unwrap_or(&0);

    assert_eq!(number_saving_2, 14);
    assert_eq!(number_saving_4, 14);
    assert_eq!(number_saving_6, 2);
    assert_eq!(number_saving_8, 4);
    assert_eq!(number_saving_10, 2);
    assert_eq!(number_saving_12, 3);
    assert_eq!(number_saving_20, 1);
    assert_eq!(number_saving_36, 1);
    assert_eq!(number_saving_38, 1);
    assert_eq!(number_saving_40, 1);
    assert_eq!(number_saving_64, 1);
}

#[test]
fn get_example_track_shortcuts_part_2() {
    let race_track = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"
        .to_string();
    let mut racetrack = convert_string_to_racetrack(race_track);

    get_distance_of_track(&mut racetrack);

    let shortcuts = find_shortcuts(&mut racetrack, 20);

    let number_saving_50 = *shortcuts.get(&50).unwrap_or(&0);
    let number_saving_52 = *shortcuts.get(&52).unwrap_or(&0);
    let number_saving_54 = *shortcuts.get(&54).unwrap_or(&0);
    let number_saving_56 = *shortcuts.get(&56).unwrap_or(&0);
    let number_saving_58 = *shortcuts.get(&58).unwrap_or(&0);
    let number_saving_60 = *shortcuts.get(&60).unwrap_or(&0);
    let number_saving_62 = *shortcuts.get(&62).unwrap_or(&0);
    let number_saving_64 = *shortcuts.get(&64).unwrap_or(&0);
    let number_saving_66 = *shortcuts.get(&66).unwrap_or(&0);
    let number_saving_68 = *shortcuts.get(&68).unwrap_or(&0);
    let number_saving_70 = *shortcuts.get(&70).unwrap_or(&0);
    let number_saving_72 = *shortcuts.get(&72).unwrap_or(&0);
    let number_saving_74 = *shortcuts.get(&74).unwrap_or(&0);
    let number_saving_76 = *shortcuts.get(&76).unwrap_or(&0);

    assert_eq!(number_saving_50, 32);
    assert_eq!(number_saving_52, 31);
    assert_eq!(number_saving_54, 29);
    assert_eq!(number_saving_56, 39);
    assert_eq!(number_saving_58, 25);
    assert_eq!(number_saving_60, 23);
    assert_eq!(number_saving_62, 20);
    assert_eq!(number_saving_64, 19);
    assert_eq!(number_saving_66, 12);
    assert_eq!(number_saving_68, 14);
    assert_eq!(number_saving_70, 12);
    assert_eq!(number_saving_72, 22);
    assert_eq!(number_saving_74, 4);
    assert_eq!(number_saving_76, 3);
}
