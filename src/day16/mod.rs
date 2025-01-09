use std::{collections::HashMap, fs, usize};

use strum::{EnumIter, IntoEnumIterator};

#[derive(EnumIter, PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy)]
struct Move {
    coordinate: Coordinate,
    points: usize,
    direction: Direction,
}

#[derive(Clone)]
struct Maze(Vec<Vec<char>>);

impl Maze {
    fn get_at_coordinate(&self, coordinate: &Coordinate) -> char {
        self.0[coordinate.y][coordinate.x]
    }

    fn rows(&self) -> usize {
        self.0[0].len()
    }

    fn columns(&self) -> usize {
        self.0.len()
    }

    fn insert_at_coordinate(&mut self, coordinate: &Coordinate, char: char) {
        self.0[coordinate.y][coordinate.x] = char
    }

    fn start_coordinate(&self) -> Coordinate {
        let mut coordinate = Coordinate { x: 0, y: 0 };
        coordinate.y = self
            .0
            .iter()
            .position(|row| {
                if let Some(x) = row.iter().position(|character| *character == 'S') {
                    coordinate.x = x;
                    return true;
                } else {
                    return false;
                }
            })
            .unwrap();

        return coordinate;
    }
}

fn read_input() -> Maze {
    let input = fs::read_to_string("src/day16/input.txt").expect("Day 16 input not present");

    let parsed_map = input.lines().map(|line| line.chars().collect()).collect();

    return Maze(parsed_map);
}

fn find_coordinate_in_direction(coordinate: &Coordinate, direction: &Direction) -> Coordinate {
    match direction {
        Direction::North => Coordinate {
            x: coordinate.x,
            y: coordinate.y - 1,
        },
        Direction::East => Coordinate {
            x: coordinate.x + 1,
            y: coordinate.y,
        },
        Direction::South => Coordinate {
            x: coordinate.x,
            y: coordinate.y + 1,
        },
        Direction::West => Coordinate {
            x: coordinate.x - 1,
            y: coordinate.y,
        },
    }
}

fn get_opposite_direction(direction: &Direction) -> Direction {
    match direction {
        Direction::North => Direction::South,
        Direction::East => Direction::West,
        Direction::South => Direction::North,
        Direction::West => Direction::East,
    }
}

fn find_possible_next_moves(
    last_move: &Move,
    input: &Maze,
    current_score: usize,
    distance_matrix: &mut HashMap<Coordinate, usize>,
) -> Vec<Move> {
    let mut moves = vec![];

    for direction in Direction::iter()
        .filter(|direction_value| *direction_value != get_opposite_direction(&last_move.direction))
    {
        let coordinate = find_coordinate_in_direction(&last_move.coordinate, &direction);
        let char_at_coordinte = input.get_at_coordinate(&coordinate);

        let points = if direction == last_move.direction {
            1
        } else {
            1001
        };

        if char_at_coordinte == '.' || char_at_coordinte == 'E' {
            let next_score = current_score + points;
            let score_at_coordinate = distance_matrix.entry(coordinate).or_insert(next_score);

            if *score_at_coordinate >= next_score {
                moves.push(Move {
                    coordinate,
                    points,
                    direction,
                });
            }
        }
    }
    // moves.sort_by(|movement_a, movement_b| movement_a.points.cmp(&movement_b.points));
    return moves;
}

fn print_board(input: &Maze) {
    let mut string_to_print = String::new();

    for y in 0..input.columns() {
        for x in 0..input.rows() {
            string_to_print.push(input.get_at_coordinate(&Coordinate { x, y }));
        }
        string_to_print.push('\n');
    }

    println!("{}", string_to_print);
}

fn move_moose(input: &mut Maze, movement: Move) {
    match movement.direction {
        Direction::North => input.insert_at_coordinate(&movement.coordinate, '^'),
        Direction::East => input.insert_at_coordinate(&movement.coordinate, '>'),
        Direction::South => input.insert_at_coordinate(&movement.coordinate, 'v'),
        Direction::West => input.insert_at_coordinate(&movement.coordinate, '<'),
    }
}

fn find_possible_path(
    input: &Maze,
    moves: HashMap<Coordinate, Move>,
    last_move: Move,
    successful_paths: &mut Vec<HashMap<Coordinate, Move>>,
    current_min: &mut usize,
    distance_matrix: &mut HashMap<Coordinate, usize>,
) {
    let current_score = calculate_score(moves.clone());
    let next_moves = find_possible_next_moves(&last_move, input, current_score, distance_matrix);

    for movement in next_moves {
        let mut moves = moves.clone();
        if let Some(_) = moves.get_key_value(&movement.coordinate) {
            continue;
        }

        moves.entry(movement.coordinate).insert_entry(movement);

        let score_for_this_block = distance_matrix
            .entry(movement.coordinate)
            .or_insert(current_score);

        if current_score > *score_for_this_block {
            println!("early termination {current_min}");
            continue;
        }

        if input.get_at_coordinate(&movement.coordinate) == 'E' {
            *current_min = current_score.clone();
            println!("found successful path with score: {}", current_score);
            successful_paths.push(moves);
            return;
        }

        find_possible_path(
            input,
            moves,
            movement,
            successful_paths,
            current_min,
            distance_matrix,
        );
    }
}

fn find_possible_paths(input: &mut Maze) -> Vec<HashMap<Coordinate, Move>> {
    let last_move = Move {
        coordinate: input.start_coordinate(),
        points: 0,
        direction: Direction::East,
    };

    let mut moves = HashMap::new();
    moves
        .entry(input.start_coordinate())
        .insert_entry(last_move);

    let mut successful_paths = vec![];

    let mut current_min = usize::MAX;
    let mut distance_matrix = HashMap::new();
    distance_matrix
        .entry(last_move.coordinate)
        .insert_entry(0 as usize);

    find_possible_path(
        input,
        moves,
        last_move,
        &mut successful_paths,
        &mut current_min,
        &mut distance_matrix,
    );

    for successhul_path in successful_paths.clone() {
        let mut input = input.clone();
        for (_, movement) in successhul_path {
            move_moose(&mut input, movement);
        }
        print_board(&input);
    }

    return successful_paths;
}

fn calculate_score(moves: HashMap<Coordinate, Move>) -> usize {
    moves.iter().map(|(_, step)| step.points).sum()
}

fn get_lowest_score(paths: Vec<HashMap<Coordinate, Move>>) -> usize {
    let mut lowest_score = usize::MAX;

    for moves in paths {
        let score = calculate_score(moves);

        if lowest_score > score {
            lowest_score = score
        }
    }

    return lowest_score;
}

fn get_next_moves(last_move: &Move) -> Vec<Move> {
    Direction::iter()
        .filter(|direction| *direction != get_opposite_direction(&last_move.direction))
        .map(|direction| {
            let coordinate = find_coordinate_in_direction(&last_move.coordinate, &direction);
            let points = if direction == last_move.direction {
                last_move.points + 1
            } else {
                last_move.points + 1001
            };
            return Move {
                coordinate,
                points,
                direction,
            };
        })
        .collect()
}

fn find_optimal_path(input: &mut Maze) -> usize {
    let start = input.start_coordinate();
    let mut score_map = HashMap::new();
    let first_move = Move {
        coordinate: start,
        points: 0,
        direction: Direction::East,
    };
    score_map.entry(start).insert_entry(first_move);

    let mut moves_to_evaluate = get_next_moves(&first_move);

    let mut current_min = usize::MAX;

    while let Some(move_to_evaluate) = moves_to_evaluate.pop() {
        let character = input.get_at_coordinate(&move_to_evaluate.coordinate);
        if character == '#' {
            continue;
        }

        if character == 'E' && move_to_evaluate.points <= current_min {
            current_min = move_to_evaluate.points;
            println!("minimum_score = {}", move_to_evaluate.points);
            continue;
        }

        let possible_next_moves = get_next_moves(&move_to_evaluate);

        for possible_next_move in possible_next_moves {
            let score_at_coordinate = score_map
                .entry(possible_next_move.coordinate)
                .or_insert(possible_next_move);

            if score_at_coordinate.points == possible_next_move.points {
                moves_to_evaluate.push(possible_next_move);
                moves_to_evaluate.sort_by(|move_a, move_b| move_b.points.cmp(&move_a.points));
            }
        }
    }
    return current_min;
}

pub fn day_16_part_1() {
    let mut input = read_input();

    // let paths = find_possible_paths(&mut input);

    let lowest_score = find_optimal_path(&mut input);

    println!("min score: {}", lowest_score)
}
