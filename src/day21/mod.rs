use core::num;
use std::fs;

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

pub fn day_21_part_1() {
    let mut numbers = read_input();

    for _ in 0..2000 {
        for number in numbers.iter_mut() {
            *number = calculate_next_secret_number(*number);
        }
    }

    let total = numbers.iter().sum::<i64>();

    println!("total of all buyrs 2000th random number: {}", total);
}
