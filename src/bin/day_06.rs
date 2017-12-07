/*
 *  Author: Samuel Remedios
 *  Advent of Code 2017 Day 06
 **/

use std::io;

fn vectorize(s: &str) -> Vec<u32> {
    s.split("\t")
        .map(|i| i.parse::<u32>().expect("non-int found"))
        .collect()
}

fn get_max(v: &Vec<u32>) -> (u32, usize) {
    v.iter()
        .enumerate()
        .map(|(x, y)| (y.clone(), x))
        .max()
        .unwrap()
}

fn redistribute(v: &mut Vec<u32>) {
    // find maximum value and its position in original vector
    let (mut max_val, _) = get_max(v);
    let idx = v.iter().position(|&x| x == max_val).unwrap();
    let n = v.len();
    // correct for values < length - 1
    let diff = if (max_val + 1) / n as u32 != 0 {
        (max_val + 1) / n as u32
    } else {
        1
    };

    // empty bank from which we took redistribution values
    v[idx] = 0;

    // start distributing from the index after max
    for i in 0..n {
        let cur_idx = (i + idx + 1) % n;

        // distribute to vector
        if max_val > 1 {
            v[cur_idx] += diff;
        } else if max_val == 1 {
            v[cur_idx] += 1;
        }

        // take away from max_val; staying >= 0
        if diff < max_val {
            max_val -= diff;
        } else {
            max_val = 0;
        }
    }
}




#[test]
fn test_vectorize() {
    assert_eq!(vectorize("0\t2\t7\t0"), vec![0, 2, 7, 0]);
}

#[test]
fn test_get_max() {
    assert_eq!(get_max(&vec![0, 2, 7, 0]), (7, 2 as usize));
}

#[test]
fn test_redistribute() {
    let mut v = vec![0, 2, 7, 0];
    redistribute(&mut v);
    assert_eq!(v, vec![2, 4, 1, 2]);
}

fn main() {
    let stdin = io::stdin();
    let mut configs = Vec::new();
    let mut count: u32 = 0;
    let mut loop_length;

    let mut string_input = String::new();
    stdin.read_line(&mut string_input).unwrap();

    let mut input = vectorize(&string_input[..].trim());

    loop {
        redistribute(&mut input);
        count += 1;
        if configs.contains(&input) {
            loop_length = configs.iter().position(|x| x == &input).unwrap();

            break;
        }
        configs.push(input.clone());
    }


    println!(
        "Part 1: {}\nPart 2: {}",
        count,
        count - (loop_length as u32 + 1) // extra 1 to account properly
    );

}
