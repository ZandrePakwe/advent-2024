use std::{fs, isize};

use regex::Regex;

fn read_input() -> FloorDescription {
    let input = fs::read_to_string("src/day14/input.txt").expect("error reading day 14 input");
    let floor = fs::read_to_string("src/day14/size.txt").expect("error reading day 14 input");
    let find_numbers_regex = Regex::new(r"-?\d+").unwrap();

    let numbers_in_floor_description = find_numbers_regex
        .find_iter(&floor)
        .map(|number| number.as_str().parse::<isize>().unwrap())
        .collect::<Vec<isize>>();

    let result = input
        .lines()
        .map(|line| {
            let values = line.split(" ").collect::<Vec<&str>>();

            let position = find_numbers_regex
                .find_iter(values[0])
                .map(|result| result.as_str().parse::<isize>().unwrap())
                .collect::<Vec<isize>>();
            let velocity = find_numbers_regex
                .find_iter(values[1])
                .map(|result| result.as_str().parse::<isize>().unwrap())
                .collect::<Vec<isize>>();

            return Robot {
                position: Coordinate {
                    x: position[0],
                    y: position[1],
                },
                velocity: Velocity {
                    dx: velocity[0],
                    dy: velocity[1],
                },
            };
        })
        .collect::<Vec<Robot>>();

    return FloorDescription {
        robots: result,
        floor: FloorDimensions {
            width: numbers_in_floor_description[0],
            height: numbers_in_floor_description[1],
        },
    };
}

#[derive(Debug, Clone)]
struct FloorDescription {
    robots: Vec<Robot>,
    floor: FloorDimensions,
}

#[derive(Debug, Copy, Clone)]
struct FloorDimensions {
    width: isize,
    height: isize,
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    position: Coordinate,
    velocity: Velocity,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Coordinate {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy)]
struct Velocity {
    dx: isize,
    dy: isize,
}

pub fn day_14_part_1() {
    let input = read_input();

    let time = 100;

    let safety_factor = calculate_safety_factor_at_time(&input, time);

    print_board_at_time(&input, time);

    println!("Safety factor: {}", safety_factor);
}

pub fn day_14_part_2() {
    let input = &read_input();

    let mut minimum_safety_factor = usize::MAX;
    let mut minimum_safety_factor_time = 0;

    for seconds in 1..1_000_000 {
        let safety_factor = calculate_safety_factor_at_time(input, seconds);

        if safety_factor < minimum_safety_factor {
            minimum_safety_factor = safety_factor;
            minimum_safety_factor_time = seconds;
            println!(
                "new minimum safety factor found at: {} seconds: {}",
                seconds, safety_factor
            );
        }

        if seconds > minimum_safety_factor_time + 10_000 {
            break;
        }
    }

    print_board_at_time(input, minimum_safety_factor_time);
}

fn calculate_safety_factor_at_time(input: &FloorDescription, time: isize) -> usize {
    let mut input = input.clone();

    input.robots = input
        .robots
        .iter_mut()
        .map(|robot| {
            robot.position = calculate_robot_position_after_time(&robot, time, &input.floor);
            return *robot;
        })
        .collect();

    let (quad_1, quad_2, quad_3, quad_4) = count_robots_in_quadrants(&input);

    let safety_factor = quad_1 * quad_2 * quad_3 * quad_4;

    return safety_factor;
}

fn print_board_at_time(input: &FloorDescription, time: isize) {
    let mut input = input.clone();

    input.robots = input
        .robots
        .iter_mut()
        .map(|robot| {
            robot.position = calculate_robot_position_after_time(&robot, time, &input.floor);
            return *robot;
        })
        .collect();

    let mut floor_string = String::new();
    for y in 0..input.floor.height {
        for x in 0..input.floor.width {
            let current_coordinate = Coordinate { x, y };
            let robots_at_coordinate = input
                .robots
                .iter()
                .filter(|robot| robot.position == current_coordinate)
                .count();

            if robots_at_coordinate == 0 {
                floor_string += ".";
                continue;
            }

            floor_string += &format!("{robots_at_coordinate}");
        }
        floor_string += "\n";
    }

    println!("{}", floor_string);
}

fn calculate_robot_position_after_time(
    robot: &Robot,
    time: isize,
    floor: &FloorDimensions,
) -> Coordinate {
    let mut x = robot.position.x + (robot.velocity.dx * time);
    let mut y = robot.position.y + (robot.velocity.dy * time);

    while x < 0 {
        x += floor.width
    }
    while y < 0 {
        y += floor.height
    }
    while x >= floor.width {
        x -= floor.width
    }
    while y >= floor.height {
        y -= floor.height
    }

    return Coordinate { x, y };
}

fn count_robots_in_quadrants(input: &FloorDescription) -> (usize, usize, usize, usize) {
    let mut quadrant_1 = 0;
    let mut quadrant_2 = 0;
    let mut quadrant_3 = 0;
    let mut quadrant_4 = 0;

    for robot in &input.robots {
        if robot.position.x * 2 == input.floor.width - 1
            || robot.position.y * 2 == input.floor.height - 1
        {
            continue;
        }
        let is_right = robot.position.x * 2 > input.floor.width;
        let is_down = robot.position.y * 2 > input.floor.height;

        if is_right && !is_down {
            quadrant_1 += 1;
        }
        if !is_right && !is_down {
            quadrant_2 += 1;
        }
        if !is_right && is_down {
            quadrant_3 += 1;
        }
        if is_right && is_down {
            quadrant_4 += 1;
        }
    }

    return (quadrant_1, quadrant_2, quadrant_3, quadrant_4);
}
