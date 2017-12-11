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

        match c {
            '!' => {
                iter.next();
                continue;
            }
            '{' => {
                if !garbage {
                    depth += 1;
                    count += depth;
                } else {
                    garbage_count += 1;
                }
            }
            '}' => {
                if !garbage {
                    depth -= 1;
                } else {
                    garbage_count += 1;
                }
            }
            '<' => {
                if !garbage {
                    garbage = true;
                } else {
                    garbage_count += 1;
                }
            }
            '>' => garbage = false,
            _ => {
                if garbage {
                    garbage_count += 1;
                }
            }
        }
    }

    println!("Total score: {}\nGarbage count: {}", count, garbage_count);
}
