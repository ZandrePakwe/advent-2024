use std::fs;

pub fn day_1() {
    let file = fs::read_to_string("src/day1/input.txt").expect("File not found");

    let lines = file.split("\n");

    let mut list_1: Vec<i64> = vec![];
    let mut list_2: Vec<i64> = vec![];

    for line in lines {
        if line.len() == 0 {
            continue;
        }
        let elements: Vec<&str> = line.split("   ").collect();
        let element_1 = elements[0].parse::<i64>().expect("Not a number");
        list_1.push(element_1);
        let element_2 = elements[1].parse::<i64>().expect("Not a number");
        list_2.push(element_2);
    }

    let mut sorted_list_1 = list_1.clone();
    sorted_list_1.sort();

    let mut sorted_list_2 = list_2.clone();
    sorted_list_2.sort();

    let mut total: i64 = 0;

    for index in 0..sorted_list_1.len() {
        let element_1 = sorted_list_1[index];
        let element_2 = sorted_list_2[index];
        let difference = (element_1 - element_2).abs();
        total += difference;
    }

    println!("difference score: {total}");

    let mut similarity_score: i64 = 0;
    for number in list_1 {
        let number_in_list_2: Vec<i64> = list_2
            .clone()
            .into_iter()
            .filter(|number_2| number_2 == &number)
            .collect();
        
        similarity_score += (number * number_in_list_2.len() as i64);
    }

    println!("similarity score: {similarity_score}");
}