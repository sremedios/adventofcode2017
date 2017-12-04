/*
 *  Author: Samuel Remedios
 *  Advent of Code 2017 Day 04
 **/

use std::io;
use std::io::prelude::*;
use std::collections::HashSet;
use std::iter::Iterator;
use std::iter::FromIterator;

fn vectorize(s: &str) -> Vec<String> {
    s.split(" ").map(|i| i.to_string()).collect()
}

fn sort_strings(v: Vec<String>) -> Vec<String> {
    let mut new_vec = Vec::new();
    for item in v {
        let mut chars: Vec<char> = item.chars().collect();
        chars.sort_by(|a, b| b.cmp(a));
        new_vec.push(String::from_iter(chars));
    }
    new_vec
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
        let vectorized_line = sort_strings(vectorize(&line.unwrap()));
        // uncomment below for answer to part 1
        //let vectorized_line = vectorize(&line.unwrap());
        let passphrases = make_set(vectorized_line.clone());
        if vectorized_line.len() == passphrases.len() {
            count += 1;
        }
    }
    println!("Solution: {}", count);

}
