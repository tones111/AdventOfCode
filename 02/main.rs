use std::io::{self, BufRead};
use std::ops::Mul;

fn main() {
    let mut total_area = 0;
    let mut total_length = 0;

    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let mut dims: Vec<usize> = line.split('x')
            .map(|n| n.parse().unwrap())
            .collect();
        dims.sort();
        let dims = dims;

        // Puzzle 1: calculate area of paper required
        // Surface area + area of smallest side
        let faces = [dims[0] * dims[1], dims[0] * dims[2], dims[1] * dims[2]];
        let area = faces.iter().fold(0, |acc, n| acc + 2 * n);
        // let min_face = faces.iter().fold(std::usize::MAX, |a, b| std::cmp::min(a, *b));
        let min_face = faces[0]; // prev sorted
        total_area += area + min_face;

        // Puzzle 2: calculate length of ribbon required
        // shortest distance around sides + volume
        // let smallest_per = dims.iter()
        //    .take(2)
        //    .collect::<Vec<&usize>>()
        //    .iter()
        //    .fold(0, |acc, n| acc + 2 * *n);
        let smallest_per = 2 * (dims[0] + dims[1]); // prev sorted
        let volume = dims.iter().fold(1, Mul::mul);
        total_length += smallest_per + volume;
    }
    println!("area: {}", total_area);
    println!("length: {}", total_length);
}
