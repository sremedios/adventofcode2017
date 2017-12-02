/*
 *  Author: Samuel Remedios
 *  Advent of Code 2017 Day 01
 **/

use std::io;

fn sum_adjacent(s: &str) -> u32 {

    let digits: Vec<u32> = s.chars()
        .map(|c| match c.to_digit(10) {
            Some(v) => v,
            None => 0,
        })
        .collect();

    let mut sum: u32 = 0;

    for i in 0..digits.len() - 1 {
        if digits[i] == digits[i + 1] {
            sum += digits[i];
        }
    }
    if digits[0] == digits[digits.len() - 1] {
        sum += digits[0];
    }
    sum
}

#[test]
fn test_sum_adjacent() {
    assert_eq!(sum_adjacent("1122"), 3);

    assert_eq!(sum_adjacent("1111"), 4);

    assert_eq!(sum_adjacent("1234"), 0);

    assert_eq!(sum_adjacent("91212129"), 9);
}

fn sum_halfway(s: &str) -> u32 {

    let digits: Vec<u32> = s.chars()
        .map(|c| match c.to_digit(10) {
            Some(v) => v,
            None => 0,
        })
        .collect();

    let mut sum: u32 = 0;

    let half_length: u32 = digits.len() as u32 / 2;
    for i in 0..digits.len() {
        let jump: u32 = (i as u32 + half_length) % digits.len() as u32;
        if digits[i] == digits[jump as usize] {
            sum += digits[i];
        }
    }
    sum
}

#[test]
fn test_sum_halfway() {
    assert_eq!(sum_halfway("1212"), 6);
    assert_eq!(sum_halfway("1221"), 0);
    assert_eq!(sum_halfway("123425"), 4);
    assert_eq!(sum_halfway("123123"), 12);
    assert_eq!(sum_halfway("12131415"), 4);

}



fn main() {
    let mut captcha = String::new();
    match io::stdin().read_line(&mut captcha) {
        Ok(_) => {}
        Err(error) => panic!("Stdin read failed. Error: {}", error),
    }
    let captcha = captcha.trim();
    let answer = sum_adjacent(&captcha);
    println!("part 1: {}", answer);

    let answer = sum_halfway(&captcha);
    println!("part 2: {}", answer);

}
