/*
 *  Author: Samuel Remedios
 *  Advent of Code 2017 Day 04
 **/

use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

fn vectorize(s: &str) -> Vec<String> {
    s.split(" ").map(|i| i.to_string()).collect()
}

fn make_set(v: Vec<String>) -> HashSet<String> {
    let mut passphrases = HashSet::new();
    for item in v {
        passphrases.insert(item.to_string());
    }
    passphrases
}

#[test]
fn test_vectorize() {
    assert_eq!(vectorize("aa bb cc dd"), vec!["aa", "bb", "cc", "dd"]);
}

fn main() {
    let stdin = io::stdin();
    let mut count: u32 = 0;

    for line in stdin.lock().lines() {
        let vectorized_line = vectorize(&line.unwrap());
        let passphrases = make_set(vectorized_line.clone());
        if vectorized_line.len() == passphrases.len() {
            count += 1;
        }
    }
    println!("Part 1: {}\nPart 2: {}", count, "incomplete");

}
