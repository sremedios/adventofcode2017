/*
 *  Author: Samuel Remedios
 *  Advent of Code 2017 Day 10
 **/

use std::io;

fn vectorize(s: &str) -> Vec<usize> {
    s.split(",").map(|i| i.parse::<usize>().unwrap()).collect()
}

fn vectorize_as_bytes(s: &str) -> Vec<usize> {
    s.bytes().map(|x| x as usize).collect()
}

fn append_tag(v: &mut Vec<usize>) {
    let tmp: Vec<usize> = vec![17, 31, 73, 47, 23];
    for item in tmp {
        v.push(item);
    }
}

fn reverse_slice(v: &mut Vec<u32>, pos: &mut usize, len: usize, skip: &mut usize) {
    let mut sublist: Vec<u32> = Vec::new();
    let length = v.len();
    for i in 0..len {
        sublist.push(v[(i + *pos) % length]);
    }
    sublist.reverse();

    for i in 0..len {
        v[(i + *pos) % length] = sublist[i];
    }
    *pos += len + *skip;
    *skip += 1;
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    match stdin.read_line(&mut input) {
        Ok(_) => {}
        Err(e) => panic!("Stdin read failed. Error: {}", e),
    }
    //let lengths: Vec<usize> = vectorize(input.trim());
    let mut lengths: Vec<usize> = vectorize_as_bytes(input.trim());
    append_tag(&mut lengths);
    let mut list: Vec<u32> = (0..256).collect();
    let mut pos: usize = 0;
    let mut skip: usize = 0;

    for _ in 0..64 {
        for item in lengths.clone() {
            reverse_slice(&mut list, &mut pos, item, &mut skip);
        }
    }
    //let part_1_answer = list[0] * list[1];

    let mut i = 0;
    let mut dense_hash: Vec<u32> = Vec::new();
    while i < 256 {
        let mut hash_val: u32 = 0;
        for j in 0..16 {
            hash_val ^= list[i + j];
        }

        dense_hash.push(hash_val);
        i += 16;
    }

    let mut part_2_answer: Vec<String> = Vec::new();
    for item in dense_hash {
        part_2_answer.push(format!("{:x}", item));
    }
    println!("Answer: {:?}", part_2_answer);

}

#[test]
fn test_vectorize() {
    assert_eq!(vectorize("1,2,3"), vec![1, 2, 3]);
}

#[test]
fn test_vectorize_as_bytes() {
    assert_eq!(vectorize_as_bytes("1,2,3"), vec![49, 44, 50, 44, 51]);
}

#[test]
fn test_append_tag() {
    let input = "1,2,3";
    let mut v = vectorize_as_bytes(input);
    append_tag(&mut v);
    assert_eq!(v, vec![49, 44, 50, 44, 51, 17, 31, 73, 47, 23]);
}

#[test]
fn test_reverse_slice() {
    let mut input = vec![0, 1, 2, 3, 4];
    let mut pos: usize = 0;
    let mut skip: usize = 0;

    reverse_slice(&mut input, &mut pos, (3 as usize), &mut skip);
    assert_eq!(input, vec![2, 1, 0, 3, 4]);
    reverse_slice(&mut input, &mut pos, (4 as usize), &mut skip);
    assert_eq!(input, vec![4, 3, 0, 1, 2]);

}
