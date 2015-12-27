use std::io;
use std::ops::Mul;

fn main() {
    let mut total_area = 0;
    let mut total_length = 0;

    let mut line = String::new();
    loop {
        line.clear();
        io::stdin().read_line(&mut line).unwrap();

        if line.len() == 0 {
                break;
        }

        let mut dims: Vec<usize> = line.trim_right().split('x')
            .map(|n| n.parse().unwrap()).collect();
        dims.sort();

        // Puzzle 1: calculate area of paper required
        let faces = [dims[0]*dims[1], dims[0]*dims[2], dims[1]*dims[2]];
        let area = faces.iter().fold(0, |acc, n| acc + 2 * n);
        let min_face = faces.iter().fold(std::usize::MAX, |a, b| std::cmp::min(a, *b));
        total_area += area + min_face;

        // Puzzle 2: calculate length of ribbon required
        let volume = dims.iter().fold(1, Mul::mul);
        let smallest_per = dims.iter().take(2).collect::<Vec<&usize>>().iter()
            .fold(0, |acc, n| acc + 2 * *n);
        total_length += volume + smallest_per;
    }
    println!("area: {}", total_area);
    println!("length: {}", total_length);
}
