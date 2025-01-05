use std::fs;

use peroxide::fuga::*;
use regex::Regex;

fn read_input() -> Vec<Matrix> {
    let input = fs::read_to_string("src/day13/input.txt").expect("day 13 input not found");
    // let matrix = matrix(vec![94, 22, 8400, 34, 67, 5400], 2, 3, Row);

    // matrix.print();

    // let matrix = matrix.rref();

    // matrix.print();

    let find_numbers_regex = Regex::new(r"\d+").unwrap();
    let mut result = vec![];

    let mut matrix_vector = vec![];
    for line in input.lines() {
        if line.len() == 0 {
            let matrix_of_equations = matrix(matrix_vector.clone(), 2, 3, Col);
            result.push(matrix_of_equations);
            matrix_vector.clear();
            continue;
        }
        for number_match in find_numbers_regex.find_iter(line) {
            matrix_vector.push(number_match.as_str().parse::<f64>().unwrap());
        }
    }

    return result;
}

pub fn day_13_part_1() {
    let matrices = read_input();

    let mut total_cost = 0.0;
    for matrix in matrices {
        let solution = matrix.rref();

        let solution_col = solution.col(2);

        let eq_1 = matrix.row(0);
        let eq_2 = matrix.row(1);

        if is_valid_solution(solution_col.clone(), eq_1, eq_2) {
            matrix.print();
            let cost = solution_col[0].round() * 3.0 + solution_col[1].round();

            total_cost += cost;
        }
    }

    println!("total cost to win all prizes: {total_cost}")
}

pub fn day_13_part_2() {
    let matrices = read_input();

    let mut total_cost = 0.0;
    for mut matrix in matrices {
        let mut result_col = matrix.col(2);

        for index in 0..result_col.len() {
            result_col[index] += 10000000000000.0;
        }

        matrix.subs_col(2, &result_col);
        let solution = matrix.rref();

        let solution_col = solution.col(2);

        let eq_1 = matrix.row(0);
        let eq_2 = matrix.row(1);

        if is_valid_solution(solution_col.clone(), eq_1, eq_2) {
            let cost = solution_col[0].round() * 3.0 + solution_col[1].round();
            total_cost += cost;
        }
    }

    println!("total cost to win all prizes: {total_cost}")
}

fn is_valid_solution(solution: Vec<f64>, eq_1: Vec<f64>, eq_2: Vec<f64>) -> bool {
    for value in &solution {
        if !is_whole_number(*value) {
            return false;
        }
    }
    let answer_1 = eq_1[0] * solution[0].round() + eq_1[1] * solution[1].round();
    let is_valid_1 = eq_1[2] == answer_1;

    let answer_2 = eq_2[0] * solution[0].round() + eq_2[1] * solution[1].round();
    let is_valid_2 = eq_2[2] == answer_2;
    return is_valid_1 && is_valid_2;
}

fn is_whole_number(number: f64) -> bool {
    return (number.round() - number).pow(2.0) < 0.001;
}
