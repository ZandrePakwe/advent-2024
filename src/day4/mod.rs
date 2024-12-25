use std::fs;

fn day_4_input() -> String {
    fs::read_to_string("src/day4/input.txt").expect("input file missing for day 4")
}

pub fn day_4_part_1() {
    let input = day_4_input();

    let lines: Vec<&str> = input.split("\n").collect();

    let mut occurences_of_xmas = 0;

    let number_of_lines = lines.len() as usize;

    let number_of_rows = lines.len();

    for index_y in 0..number_of_lines {
        let line_at_y: &str = lines[index_y];
        for index_x in 0..line_at_y.len() {
            if is_xmas_north(&lines, index_x, index_y) {
                // println!("XMAS found at ({index_x},{index_y}) in North Direction");
                occurences_of_xmas += 1;
            }
            if is_xmas_north_east(&lines, index_x, index_y, number_of_rows) {
                // println!("XMAS found at ({index_x},{index_y}) in North East Direction");
                occurences_of_xmas += 1;
            }
            if is_xmas_east(&lines, index_x, index_y, number_of_rows) {
                // println!("XMAS found at ({index_x},{index_y}) in East Direction");
                occurences_of_xmas += 1;
            }
            if is_xmas_south_east(&lines, index_x, index_y, number_of_lines, number_of_rows) {
                // println!("XMAS found at ({index_x},{index_y}) in South East Direction");
                occurences_of_xmas += 1;
            }
            if is_xmas_south(&lines, index_x, index_y, number_of_lines) {
                // println!("XMAS found at ({index_x},{index_y}) in South Direction");
                occurences_of_xmas += 1;
            }
            if is_xmas_south_west(&lines, index_x, index_y, number_of_lines) {
                // println!("XMAS found at ({index_x},{index_y}) in South West Direction");
                occurences_of_xmas += 1;
            }
            if is_xmas_west(&lines, index_x, index_y) {
                // println!("XMAS found at ({index_x},{index_y}) in West Direction");
                occurences_of_xmas += 1;
            }
            if is_xmas_north_west(&lines, index_x, index_y) {
                // println!("XMAS found at ({index_x},{index_y}) in North West Direction");
                occurences_of_xmas += 1;
            }
        }
    }

    println!("XMAS occurs {occurences_of_xmas} times")
}

pub fn day_4_part_2() {
    let input = day_4_input();

    let lines: Vec<&str> = input.split("\n").collect();

    let mut occurences_of_xmas = 0;

    let number_of_lines = lines.len() as usize;

    for index_y in 1..number_of_lines - 1 {
        let line_at_y: &str = lines[index_y];
        for index_x in 1..line_at_y.len() - 1 {
            let mut mas_diagonals = 0;
            if check_for_char_at_index(&lines, 'A', index_x, index_y) {
                if check_for_char_at_index(&lines, 'M', index_x - 1, index_y - 1)
                    && check_for_char_at_index(&lines, 'S', index_x + 1, index_y + 1)
                {
                    mas_diagonals += 1;
                }
                if check_for_char_at_index(&lines, 'S', index_x - 1, index_y - 1)
                    && check_for_char_at_index(&lines, 'M', index_x + 1, index_y + 1)
                {
                    mas_diagonals += 1;
                }

                if check_for_char_at_index(&lines, 'M', index_x - 1, index_y + 1)
                    && check_for_char_at_index(&lines, 'S', index_x + 1, index_y - 1)
                {
                    mas_diagonals += 1;
                }
                if check_for_char_at_index(&lines, 'S', index_x - 1, index_y + 1)
                    && check_for_char_at_index(&lines, 'M', index_x + 1, index_y - 1)
                {
                    mas_diagonals += 1;
                }

                if mas_diagonals == 2 {
                    occurences_of_xmas += 1;
                }
            }
        }
    }
    println!("Occurences of X-MAS : {occurences_of_xmas}");
}

fn check_for_char_at_index(lines: &Vec<&str>, char: char, index_x: usize, index_y: usize) -> bool {
    lines[index_x].chars().nth(index_y) == Some(char)
}

fn is_xmas_north(lines: &Vec<&str>, index_x: usize, index_y: usize) -> bool {
    if !is_north_possible(index_y, "XMAS") {
        return false;
    }
    check_for_char_at_index(lines, 'X', index_x, index_y)
        && check_for_char_at_index(lines, 'M', index_x, index_y - 1)
        && check_for_char_at_index(lines, 'A', index_x, index_y - 2)
        && check_for_char_at_index(lines, 'S', index_x, index_y - 3)
}

fn is_xmas_north_east(lines: &Vec<&str>, index_x: usize, index_y: usize, x_max: usize) -> bool {
    if !is_north_possible(index_y, "XMAS") {
        return false;
    }

    if !is_east_possible(index_x, x_max, "XMAS") {
        return false;
    }
    check_for_char_at_index(lines, 'X', index_x, index_y)
        && check_for_char_at_index(lines, 'M', index_x + 1, index_y - 1)
        && check_for_char_at_index(lines, 'A', index_x + 2, index_y - 2)
        && check_for_char_at_index(lines, 'S', index_x + 3, index_y - 3)
}

fn is_xmas_east(lines: &Vec<&str>, index_x: usize, index_y: usize, x_max: usize) -> bool {
    if !is_east_possible(index_x, x_max, "XMAS") {
        return false;
    }
    check_for_char_at_index(lines, 'X', index_x, index_y)
        && check_for_char_at_index(lines, 'M', index_x + 1, index_y)
        && check_for_char_at_index(lines, 'A', index_x + 2, index_y)
        && check_for_char_at_index(lines, 'S', index_x + 3, index_y)
}

fn is_xmas_south_east(
    lines: &Vec<&str>,
    index_x: usize,
    index_y: usize,
    y_max: usize,
    x_max: usize,
) -> bool {
    if !is_south_possible(index_y, y_max, "XMAS") {
        return false;
    }

    if !is_east_possible(index_x, x_max, "XMAS") {
        return false;
    }
    check_for_char_at_index(lines, 'X', index_x, index_y)
        && check_for_char_at_index(lines, 'M', index_x + 1, index_y + 1)
        && check_for_char_at_index(lines, 'A', index_x + 2, index_y + 2)
        && check_for_char_at_index(lines, 'S', index_x + 3, index_y + 3)
}

fn is_xmas_south(lines: &Vec<&str>, index_x: usize, index_y: usize, y_max: usize) -> bool {
    if !is_south_possible(index_y, y_max, "MAS") {
        return false;
    }
    check_for_char_at_index(lines, 'X', index_x, index_y)
        && check_for_char_at_index(lines, 'M', index_x, index_y + 1)
        && check_for_char_at_index(lines, 'A', index_x, index_y + 2)
        && check_for_char_at_index(lines, 'S', index_x, index_y + 3)
}

fn is_xmas_south_west(lines: &Vec<&str>, index_x: usize, index_y: usize, y_max: usize) -> bool {
    if !is_south_possible(index_y, y_max, "XMAS") {
        return false;
    }

    if !is_west_possible(index_x, "XMAS") {
        return false;
    }
    check_for_char_at_index(lines, 'X', index_x, index_y)
        && check_for_char_at_index(lines, 'M', index_x - 1, index_y + 1)
        && check_for_char_at_index(lines, 'A', index_x - 2, index_y + 2)
        && check_for_char_at_index(lines, 'S', index_x - 3, index_y + 3)
}

fn is_xmas_north_west(lines: &Vec<&str>, index_x: usize, index_y: usize) -> bool {
    if !is_north_possible(index_y, "XMAS") {
        return false;
    }

    if !is_west_possible(index_x, "XMAS") {
        return false;
    }
    check_for_char_at_index(lines, 'X', index_x, index_y)
        && check_for_char_at_index(lines, 'M', index_x - 1, index_y - 1)
        && check_for_char_at_index(lines, 'A', index_x - 2, index_y - 2)
        && check_for_char_at_index(lines, 'S', index_x - 3, index_y - 3)
}

fn is_xmas_west(lines: &Vec<&str>, index_x: usize, index_y: usize) -> bool {
    if !is_west_possible(index_x, "XMAS") {
        return false;
    }
    check_for_char_at_index(lines, 'X', index_x, index_y)
        && check_for_char_at_index(lines, 'M', index_x - 1, index_y)
        && check_for_char_at_index(lines, 'A', index_x - 2, index_y)
        && check_for_char_at_index(lines, 'S', index_x - 3, index_y)
}

fn is_north_possible(index_y: usize, word: &str) -> bool {
    return index_y >= word.len() - 1;
}

fn is_east_possible(index_x: usize, x_max: usize, word: &str) -> bool {
    return index_x <= x_max - word.len();
}

fn is_south_possible(index_y: usize, y_max: usize, word: &str) -> bool {
    return index_y <= y_max - word.len();
}

fn is_west_possible(index_x: usize, word: &str) -> bool {
    return index_x >= word.len() - 1;
}
