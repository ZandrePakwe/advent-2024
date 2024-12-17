use std::fs;

fn read_input() -> (Vec<[i32; 2]>, Vec<Vec<i32>>) {
    let input = fs::read_to_string("src/day5/input.txt").expect("input file not found");
    let mut rules = vec![];
    let mut updates = vec![];

    let mut is_in_updates = false;
    for line in input.split("\n") {
        if line == "" {
            is_in_updates = true;
            continue;
        }

        if is_in_updates {
            let update_pages: Vec<i32> = line
                .split(",")
                .map(|number_string| number_string.parse::<i32>().expect("not  number"))
                .collect();
            updates.push(update_pages);
            continue;
        }

        let parsed_numbers: Vec<i32> = line
            .split("|")
            .map(|number_stribng| number_stribng.parse::<i32>().expect("Incorrect formatting"))
            .collect();

        rules.push([parsed_numbers[0], parsed_numbers[1]]);
    }

    return (rules, updates);
}

pub fn day_5_part_1() {
    let (rules, updates) = read_input();

    let mut total_of_middle_page_numbers = 0;
    for update in updates {
        println!("{:?}", update);
        let mut has_broken_a_rule = false;
        for rule in &rules {
            if !is_following_rule(*rule, update.clone()) {
                println!("^ is breaking rule {:?}", rule);
                has_broken_a_rule = true;
                break;
            }
        }
        if !has_broken_a_rule {
            println!("^ is following all rules");
            let middle_number = update[update.len() / 2];
            println!("{middle_number}");
            total_of_middle_page_numbers += middle_number;
        }
    }

    println!("{total_of_middle_page_numbers}");
}

fn is_following_rule(rule: [i32; 2], update: Vec<i32>) -> bool {
    if !update.contains(&rule[0]) || !update.contains(&rule[1]) {
        return true;
    }

    let mut is_first_rule_first = false;
    for page in update {
        if page == rule[0] {
            is_first_rule_first = true;
            continue;
        }

        if page == rule[1] && is_first_rule_first {
            return true;
        }
    }

    return false;
}
