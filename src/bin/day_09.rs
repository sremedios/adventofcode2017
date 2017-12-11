/*
 *  Author: Samuel Remedios
 *  Advent of Code 2017 Day 09
 **/

use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut count: u32 = 0;
    let mut garbage_count: u32 = 0;
    let mut garbage: bool = false;
    let mut depth: u32 = 0;

    let mut iter = stdin.lock().bytes();

    while let Some(b) = iter.next() {
        let c = b.unwrap() as char;

        if c != '!' && c != '>' && garbage {
            garbage_count += 1;
            continue;
        }

        match c {
            '!' => {
                iter.next();
                continue;
            }
            '{' => {
                depth += 1;
                count += depth;
            }
            '}' => depth -= 1,
            '<' => garbage = true,
            '>' => garbage = false,
            _ => (),

        }
    }

    println!("Total score: {}\nGarbage count: {}", count, garbage_count);
}
