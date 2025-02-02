use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[test]
fn mix_produces_correct_response() {
    assert_eq!(mix(42, 15), 37)
}

#[test]
fn prune_produces_correct_response() {
    assert_eq!(prune(100_000_000), 16_113_920)
}

#[test]
fn get_next_secret_number_correctly() {
    assert_eq!(calculate_next_secret_number(123), 15887950)
}

#[test]
fn get_nth_secret_number_correctly() {
    let mut number = 123;
    for _ in 0..10 {
        number = calculate_next_secret_number(number);
    }
    assert_eq!(number, 5908254)
}

#[test]
fn part_1_sample() {
    let mut buyers = vec![1, 10, 100, 2024];

    for _ in 0..2_000 {
        for buyer in buyers.iter_mut() {
            *buyer = calculate_next_secret_number(*buyer);
        }
    }

    let total = buyers.iter().sum::<i64>();
    assert_eq!(total, 37327623);
}

#[test]
fn get_ones_correctly() {
    let number = 8914;
    let ones = calculate_selling_price(number);
    assert_eq!(ones, 4)
}

#[test]
fn sample_part_2() {
    let seeds = vec![1, 2, 3, 2024];
    let mut total = 0;
    let selling_price = find_sequences(seeds);
    total += selling_price;

    assert_eq!(total, 23)
}

fn read_input() -> Vec<i64> {
    let text = fs::read_to_string("src/day21/input.txt").expect("day 21 input not present");

    let buyers = text.lines().map(|line| line.parse().unwrap()).collect();

    return buyers;
}

fn mix(number: i64, secret_number: i64) -> i64 {
    number ^ secret_number
}

fn prune(secret_number: i64) -> i64 {
    secret_number % 16777216
}

fn calculate_next_secret_number(current_number: i64) -> i64 {
    let mut secret_number = current_number;
    let step_1 = secret_number * 64;
    secret_number = mix(step_1, secret_number);
    secret_number = prune(secret_number);
    let step_2 = secret_number / 32;
    secret_number = mix(step_2, secret_number);
    secret_number = prune(secret_number);
    let step_3 = secret_number * 2048;
    secret_number = mix(step_3, secret_number);
    secret_number = prune(secret_number);

    secret_number
}

fn calculate_selling_price(secret_number: i64) -> i64 {
    let rounded_without_ones = secret_number / 10;
    let full_without_ones = rounded_without_ones * 10;
    let ones = secret_number - full_without_ones;
    return ones;
}

#[derive(PartialEq, Eq, Clone, Hash, PartialOrd, Debug)]
struct Sequence(i64, i64, i64, i64);

impl Sequence {
    fn move_sequence_down(&mut self, price: i64) {
        self.0 = self.1;
        self.1 = self.2;
        self.2 = self.3;
        self.3 = price;
    }
}

fn find_sequences(numbers: Vec<i64>) -> u64 {
    let mut result = HashMap::new();

    let mut max = 0;

    for number in numbers {
        let mut sequence = Sequence(0, 0, 0, 0);
        let mut previous_price = calculate_selling_price(number);
        let mut number = number;
        let mut existing_sequences = HashSet::new();
        for index in 0..=2000 {
            number = calculate_next_secret_number(number);
            let price = calculate_selling_price(number);
            let delta = price - previous_price;
            sequence.move_sequence_down(delta);
            previous_price = price;

            if index > 3 && !existing_sequences.contains(&sequence) {
                existing_sequences.insert(sequence.clone());
                let total_price_for_sequence_so_far = result.entry(sequence.clone()).or_insert(0);
                *total_price_for_sequence_so_far += price as u64;

                if *total_price_for_sequence_so_far > max {
                    max = *total_price_for_sequence_so_far;
                }
            }
        }
    }

    return max;
}

pub fn day_21_part_1() {
    let mut numbers = read_input();

    for _ in 0..2000 {
        for number in numbers.iter_mut() {
            *number = calculate_next_secret_number(*number);
        }
    }

    let total = numbers.iter().sum::<i64>();

    println!("total of all buyers 2000th random number: {}", total);
}

pub fn day_21_part_2() {
    let numbers = read_input();

    let total = find_sequences(numbers);

    println!("Maximum price we can get: {}", total);
}
