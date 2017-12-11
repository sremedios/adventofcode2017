/*
 *  Author: Samuel Remedios
 *  Advent of Code 2017 Day 07
 **/
extern crate regex;

use std::io;
use std::io::prelude::*;
use regex::Regex;

#[derive(Debug)]
#[derive(PartialEq)]
struct Disc<'a> {
    name: &'a str,
    weight: u32,
    holding_discs: Vec<&'a str>,
}

fn create_disc<'a>(input_line: &'a str, reg: &Regex) -> Disc<'a> {
    let mut d = Disc {
        name: "",
        weight: 0,
        holding_discs: Vec::new(),
    };
    let parsed_input: Vec<&str> = Vec::new();

    // TODO figure out how to simply return the parsed-out input with regex

    for cap in reg.captures_iter(input_line) {
        println!("{:?}", &cap[0]);
        //parsed_input.push(&c.unwrap());
    }
    /*

    d.name = parsed_input[0];
    d.weight = parsed_input[1].to_string().parse::<u32>().unwrap();
    if parsed_input.len() > 2 {
        for i in 2..parsed_input.len() {
            d.holding_discs.push(parsed_input[i]);
        }
    }

    // return the disc struct
    */
    d
}

#[test]
fn test_create_disc() {
    let reg = Regex::new(r"[[::alphanum::]]+").unwrap();
    assert_eq!(
        create_disc("asdf, (49), a b c", &reg),
        Disc {
            name: "asdf",
            weight: 49,
            holding_discs: vec!["a", "b", "c"],
        }
    );
}

#[test]
fn test_regex() {
    let reg = Regex::new(r"[[::alphanum::]]+").unwrap();
    assert!(reg.is_match("asdf 3094f s"));
}

fn main() {
    let stdin = io::stdin();
    /*
    let mut count = 0;

    let mut bottoms: Vec<String> = Vec::new();
    let mut non_bottoms: Vec<String> = Vec::new();

    for line in stdin.lock().lines() {
        let line: String = line.unwrap();
        let line: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();

        if line.contains(&"->".to_string()) {
            bottoms.push(line[0].clone());

            for i in 3..line.len() {
                non_bottoms.push(line[i].clone());
            }

            count += 1;
        }


        /*
        for i in 0..bottoms.len() {
            if non_bottoms.contains(&bottoms[i]) {
                let idx = bottoms.iter().position(|x| x == &bottoms[i]).unwrap();
                bottoms.remove(idx);
            }
        }**/

        /*
        for b in &mut bottoms {
            if non_bottoms.contains(&b) {
                let idx = &bottoms.iter().position(|&x| &x == b).unwrap();
                &bottoms.remove(*idx);
            }
        }**/
    }
    let mut answer: String = "sam".to_string();
    for i in 0..bottoms.len() {
        if !non_bottoms.contains(&bottoms[i]) {
            answer = bottoms[i].clone();
            println!("{:?}", bottoms[i]);
        }
    }
    //println!("{:?}", bottoms);
    println!("{:?}", non_bottoms.contains(&answer));
    println!("{}", count);
    **/
}
