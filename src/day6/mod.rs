use std::{
    collections::HashSet,
    fs,
    sync::{atomic::AtomicUsize, Arc},
    thread,
};

fn read_input() -> Vec<Vec<char>> {
    let text = fs::read_to_string("src/day6/input.txt").expect("input file not found");

    let vector_2d: Vec<Vec<char>> = text
        .split("\n")
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    return vector_2d;
}

pub fn day_6_part_1() {
    let input = read_input();

    if let GuardResultType::Exit(location_history) = calculate_guard_route(input) {
        println!("The guard took {} steps", location_history.len())
    }
}

pub fn day_6_part_2() {
    let input = read_input();
    let mut loops = 0;

    if let GuardResultType::Exit(location_history) = calculate_guard_route(input.clone()) {
        for coordinate in location_history {
            let mut test = input.clone();
            test[coordinate.y][coordinate.x] = '#';
            if let GuardResultType::Loop = calculate_guard_route(test) {
                loops += 1;
                continue;
            }
        }
    }

    println!("Total options for loops: {loops}");
}

pub fn day_6_part_2_multithread() {
    let input = read_input();

    let loops = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    if let GuardResultType::Exit(location_history) = calculate_guard_route(input.clone()) {
        for coordinate in location_history {
            let mut test = input.clone();
            if test[coordinate.y][coordinate.x] != '^' {
                test[coordinate.y][coordinate.x] = '#';
                let counter = Arc::clone(&loops);
                let handle = thread::spawn(move || {
                    if let GuardResultType::Loop = calculate_guard_route(test.clone()) {
                        counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    }
                });

                handles.push(handle);
            }
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!(
        "Total options for loops: {}",
        loops.load(std::sync::atomic::Ordering::SeqCst)
    );
}

fn calculate_guard_route(mut input: Vec<Vec<char>>) -> GuardResultType {
    let mut x = 0;

    let mut y = input
        .iter()
        .position(|line| {
            let is_starting_in_this_line = line.iter().position(|char| *char == '^');
            if let Some(index) = is_starting_in_this_line {
                x = index;
                return true;
            }
            return false;
        })
        .expect("starting position not found");

    let mut location_history_hash: HashSet<LocationHistory> = HashSet::new();

    location_history_hash.insert(LocationHistory { x, y });

    let mut current_direction = MoveDirection::North;

    let mut movement_history_hash: HashSet<MoveHistory> = HashSet::new();

    loop {
        // if movement_history.iter().any(|history| {
        //     history.x == x
        //         && history.y == y
        //         && history.direction as usize == current_direction as usize
        // }) {
        //     return GuardResultType::Loop;
        // }
        if movement_history_hash.iter().any(|history| {
            *history
                == MoveHistory {
                    x,
                    y,
                    direction: current_direction,
                }
        }) {
            return GuardResultType::Loop;
        }
        match can_move_in_direction(&x, &y, &input, &current_direction) {
            MovementType::Move('.') => {
                movement_history_hash.insert(MoveHistory {
                    x,
                    y,
                    direction: current_direction,
                });

                location_history_hash.insert(LocationHistory { x, y });

                (x, y, input) = move_guard(x, y, input, current_direction);
            }
            MovementType::Move('X') => {
                movement_history_hash.insert(MoveHistory {
                    x,
                    y,
                    direction: current_direction,
                });

                location_history_hash.insert(LocationHistory { x, y });

                (x, y, input) = move_guard(x, y, input, current_direction);
            }
            MovementType::Move(_) => current_direction = rotate_90_deg(&current_direction),
            MovementType::Exit => {
                input = exit_from_floor(x, y, input);
                location_history_hash.insert(LocationHistory { x, y });
                break;
            }
        }
    }

    return GuardResultType::Exit(location_history_hash);
}

#[derive(Clone, Copy, PartialEq, Hash, Eq, Debug)]
enum MoveDirection {
    North,
    East,
    South,
    West,
}

enum GuardResultType {
    Exit(HashSet<LocationHistory>),
    Loop,
}

enum MovementType {
    Move(char),
    Exit,
}
#[derive(Hash, Eq, PartialEq, Debug)]
struct MoveHistory {
    x: usize,
    y: usize,
    direction: MoveDirection,
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct LocationHistory {
    x: usize,
    y: usize,
}

fn next_index_in_direction(x: &usize, y: &usize, direction: &MoveDirection) -> (isize, isize) {
    let x = *x as isize;
    let y = *y as isize;
    match direction {
        MoveDirection::North => return (x, y - 1),
        MoveDirection::East => return ((x + 1), y),
        MoveDirection::South => return (x, (y + 1)),
        MoveDirection::West => return ((x - 1), y),
    }
}

fn can_move_in_direction(
    x: &usize,
    y: &usize,
    input: &Vec<Vec<char>>,
    direction: &MoveDirection,
) -> MovementType {
    let (x_new, y_new) = next_index_in_direction(x, y, direction);
    let size_of_floor = input.len() as isize - 1;
    if x_new > size_of_floor || x_new < 0 || y_new > size_of_floor || y_new < 0 {
        return MovementType::Exit;
    }
    let mut potential_board = input.clone();
    potential_board[*y][*x] = 'X';
    potential_board[y_new as usize][x_new as usize] = '*';

    if let Some(column) = input.get(y_new as usize) {
        if let Some(character) = column.get(x_new as usize) {
            return MovementType::Move(*character);
        };
    }
    return MovementType::Exit;
}

fn move_guard(
    x: usize,
    y: usize,
    mut input: Vec<Vec<char>>,
    direction: MoveDirection,
) -> (usize, usize, Vec<Vec<char>>) {
    input[y][x] = 'X';
    let (x, y) = next_index_in_direction(&x, &y, &direction);
    return (x as usize, y as usize, input);
}

fn exit_from_floor(x: usize, y: usize, mut input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    input[y][x] = 'X';
    return input;
}

fn rotate_90_deg(direction: &MoveDirection) -> MoveDirection {
    match direction {
        MoveDirection::North => MoveDirection::East,
        MoveDirection::East => MoveDirection::South,
        MoveDirection::South => MoveDirection::West,
        MoveDirection::West => MoveDirection::North,
    }
}

// fn print_board(input: &Vec<Vec<char>>) {
//     let mut final_string = "".to_string();
//     for column in input {
//         for character in column {
//             final_string += &character.to_string();
//         }
//         final_string += "\n";
//     }

//     println!("{final_string}");
// }
