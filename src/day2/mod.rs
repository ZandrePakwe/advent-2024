use std::fs;

pub fn day_2() {
    let file = fs::read_to_string("src/day2/input.txt").expect("input file not found!");

    let reports = file.split("\n");

    let mut number_of_safe_reports_part_1 = 0;
    let mut number_of_safe_reports_part_2 = 0;

    for report in reports {
        let readings: Vec<i32> = report
            .split(" ")
            .filter(|reading| reading.parse::<i32>().is_ok())
            .map(|reading| reading.parse::<i32>().expect("Error"))
            .collect();

        if readings.len() == 0 {
            continue;
        }

        let is_correct_part_1 = is_correct_report(&readings);
        let mut is_correct_part_2 = true;

        if !is_correct_part_1 {
            is_correct_part_2 = false;
            for removed_index in 0..readings.len() {
                let mut modified_readings = readings.clone();
                modified_readings.remove(removed_index);

                let is_pass_on_parse = is_correct_report(&modified_readings);

                if is_pass_on_parse {
                    is_correct_part_2 = is_pass_on_parse;
                }
            }
        }

        if is_correct_part_1 {
            number_of_safe_reports_part_1 += 1;
        }

        if is_correct_part_2 {
            number_of_safe_reports_part_2 += 1;
        }
    }

    println!("{number_of_safe_reports_part_1} {number_of_safe_reports_part_2}");
}

fn is_correct_report(readings: &Vec<i32>) -> bool {
    let mut is_correct = true;
    let change_direction = readings[1] - readings[0];

    for index in 0..readings.len() - 1 {
        let difference = readings[index + 1] - readings[index];
        if difference.abs() > 3 || difference.abs() < 1 {
            is_correct = false;

            continue;
        }

        if (difference > 0 && change_direction < 0) || (difference < 0 && change_direction > 0) {
            is_correct = false;

            continue;
        }
    }

    return is_correct;
}