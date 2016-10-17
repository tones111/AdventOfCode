use std::io::{self, Read};

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
}

fn main() {

    let mut basement = 0;
    let mut floor = 0;

    for (i, d) in io::stdin()
        .bytes()
        .filter_map(|d| {
            match d.unwrap() as char {
                '(' => Some(Direction::Up),
                ')' => Some(Direction::Down),
                _ => None,
            }
        })
        .enumerate() {

        floor += match d {
            Direction::Up => 1,
            Direction::Down => -1,
        };

        if floor == -1 && basement == 0 {
            basement = i + 1;
            println!("basement: {}", basement);
        }
    }
    println!("floor: {}", floor);
}
