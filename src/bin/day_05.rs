/*
 *  Author: Samuel Remedios
 *  Advent of Code 2017 Day 05
 *  Getting lazier and the code is becoming just one big main function :/
 **/
use std::io;
use std::io::prelude::*;

fn main() {

    let stdin = io::stdin();
    let mut input = "".to_string();
    for line in stdin.lock().lines() {
        input = [input, line.unwrap(), "\n".to_string()].concat();
    }


    let mut v: Vec<i32> = input
        .lines()
        .map(|i| i.parse::<i32>().expect("non-integer input detected"))
        .collect();

    let mut i = v[0];
    let mut count = 0;

    while (i as usize) < v.len() {
        let j = i;
        i += v[i as usize];

        if v[j as usize] >= 3 {
            v[j as usize] -= 1;
        } else {
            v[j as usize] += 1;
        }
        count += 1;
    }

    println!("{}", count);

}
