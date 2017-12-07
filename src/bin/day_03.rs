/*
 *  Author: Samuel Remedios
 *  Advent of Code 2017 Day 03
 **/

fn nearest_odd_square_above(num: f64) -> f64 {
    let mut val = num.sqrt().ceil();
    if val % 2. == 0. {
        val += 1.;
    }
    val.powi(2)
}

fn get_corners(area: f64) -> Vec<f64> {
    vec![
        area - (area.sqrt() - 1.).floor() * 0.,
        area - (area.sqrt() - 1.).floor() * 1.,
        area - (area.sqrt() - 1.).floor() * 2.,
        area - (area.sqrt() - 1.).floor() * 3.,
    ]
}

fn get_interval(input: f64) -> (f64, f64) {
    let area = nearest_odd_square_above(input);
    let corners = get_corners(area);
    for i in 0..corners.len() {
        if input > corners[i] {
            return (corners[i], corners[i - 1]);
        }
    }
    // default case is right-hand side
    (corners[0 as usize], corners[corners.len() - 1 as usize])
}

fn distance_to_center(input: f64, (lower, upper): (f64, f64), area: f64) -> f64 {
    let dist_to_mid = (area.sqrt() - 1.) / 2.;

    let mid = if lower < upper {
        (lower + upper) / 2.
    } else {
        upper - dist_to_mid
    };

    (input - mid).abs() + dist_to_mid
}


#[test]
fn test_distance_to_center() {
    assert_eq!(distance_to_center(23., (21., 25.), 25.), 2.);
    assert_eq!(distance_to_center(12., (25., 13.), 25.), 3.);
}

#[test]
fn test_get_interval() {
    assert_eq!(get_interval(8.), (7., 9.));
    assert_eq!(get_interval(11.), (25., 13.));
}

#[test]
fn test_nearest_odd_square_above() {
    assert_eq!(nearest_odd_square_above(20.), 25.);
    assert_eq!(nearest_odd_square_above(21.), 25.);
    assert_eq!(nearest_odd_square_above(50.), 81.);
}

#[test]
fn test_get_corners() {
    assert_eq!(get_corners(9.), vec![9., 7., 5., 3.]);
    assert_eq!(get_corners(25.), vec![25., 21., 17., 13.]);
    assert_eq!(get_corners(49.), vec![49., 43., 37., 31.]);
}

fn main() {
    // first calculate closest square
    let input: f64 = 277678.;

    let area = nearest_odd_square_above(input);
    let corners = get_corners(area);
    let interval = get_interval(input);

    println!("Part 1 {}", distance_to_center(input, interval, area));

    let mut matrix: [[u32; max_size]; max_size] = [[0; max_size]; max_size];

}
