use std::fs;

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

    if let GuardResultType::Exit(total_steps) = calculate_guard_route(input) {
        println!("The guard took {} steps", total_steps)
    }
}

pub fn day_6_part_2() {
    let input = read_input();

    let mut loops = 0;
    let mut total_tries = 0;
    for y in 0..input.len() {
        for x in 0..input.len() {
            total_tries += 1;
            let character = input[y][x];
            if character != '#' && character != '^' {
                let mut test = input.clone();
                test[y][x] = '#';

                if let GuardResultType::Loop = calculate_guard_route(test) {
                    loops += 1;
                    continue;
                }
            }
        }
    }

    println!("Total options for loops: {loops}");
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

    println!("found ^ at index ({},{}): {}", x, y, input[y][x]);

    let mut current_direction = MoveDirection::North;
    let mut movement_history: Vec<MoveHistory> = vec![];

    loop {
        if movement_history.iter().any(|history| {
            history.x == x
                && history.y == y
                && history.direction as usize == current_direction as usize
        }) {
            return GuardResultType::Loop;
        }
        match can_move_in_direction(&x, &y, &input, &current_direction) {
            MovementType::Move('.') => {
                movement_history.push(MoveHistory {
                    x,
                    y,
                    direction: current_direction,
                });

                (x, y, input) = move_guard(x, y, input, current_direction);
            }
            MovementType::Move('X') => {
                movement_history.push(MoveHistory {
                    x,
                    y,
                    direction: current_direction,
                });

                (x, y, input) = move_guard(x, y, input, current_direction);
            }
            MovementType::Move(_) => current_direction = rotate_90_deg(&current_direction),
            MovementType::Exit => {
                input = exit_from_floor(x, y, input);
                break;
            }
        }
    }

    let total_steps = count_steps(input);

    return GuardResultType::Exit(total_steps);
}

#[derive(Clone, Copy)]
enum MoveDirection {
    North,
    East,
    South,
    West,
}

enum GuardResultType {
    Exit(usize),
    Loop,
}

enum MovementType {
    Move(char),
    Exit,
}

struct MoveHistory {
    x: usize,
    y: usize,
    direction: MoveDirection,
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

fn count_steps(input: Vec<Vec<char>>) -> usize {
    let mut total: usize = 0;
    let mut final_string = "".to_string();
    for column in input {
        for character in column {
            final_string += &character.to_string();
            if character == 'X' {
                total += 1;
            }
        }
        final_string += "\n";
    }

    // println!("{final_string}");

    return total;
}
