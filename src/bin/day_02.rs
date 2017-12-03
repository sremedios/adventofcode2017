/*
 *  Author: Samuel Remedios
 *  Advent of Code 2017 Day 01
 **/

use std::io;
use std::io::prelude::*;

fn vectorize(s: &str) -> Vec<u32> {
    s.split("\t")
        .map(|i| i.parse::<u32>().expect("Non-integer input"))
        .collect()
}


fn get_line_difference(data: Vec<u32>) -> u32 {
    let mut largest = data[0 as usize];
    let mut smallest = data[0 as usize];

    for item in data {
        if item > largest {
            largest = item as u32;
        }
        if item < smallest {
            smallest = item as u32;
        }
    }
    largest - smallest
}

fn get_even_diff(data: Vec<u32>) -> u32 {
    // for each value, compare against all others if less than
    for i in 0..data.len() {
        for j in 0..data.len() {
            let a = data[i as usize];
            let b = data[j as usize];
            if a != b {
                if a % b == 0 {
                    return a / b;
                }
                if b % a == 0 {
                    return b / a;
                }
            }
        }
    }
    0 // return 0 as default case
}


#[test]
fn test_checksum() {
    assert_eq!(get_line_difference(vectorize("5\t1\t9\t5".trim())), 8);
    assert_eq!(get_line_difference(vectorize("7\t5\t3\t".trim())), 4);
    assert_eq!(get_line_difference(vectorize("2\t4\t6\t8".trim())), 6);
}
#[test]
fn test_even_checksum() {
    assert_eq!(get_even_diff(vectorize("5\t9\t2\t8".trim())), 4);
    assert_eq!(get_even_diff(vectorize("9\t4\t7\t3".trim())), 3);
    assert_eq!(get_even_diff(vectorize("3\t8\t6\t5".trim())), 2);
}

fn main() {
    let stdin = io::stdin();

    let mut checksum = 0;
    let mut evens_checksum = 0;

    for line in stdin.lock().lines() {
        let vectorized_line = vectorize(&line.unwrap());
        checksum += get_line_difference(vectorized_line.clone());
        evens_checksum += get_even_diff(vectorized_line.clone());
    }
    println!("Part 1: {}\nPart 2: {}", checksum, evens_checksum);

}
