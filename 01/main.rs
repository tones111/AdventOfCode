use std::io::{self, Read};

fn main() {

    let mut basement = 0;
    let mut floor = 0;

    for (i, b) in io::stdin().bytes().enumerate() {
        floor += match b.unwrap() as char {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };

        if floor == -1 && basement == 0 {
            basement = i + 1;
            println!("basement: {}", basement);
        }
    }
    println!("floor: {}", floor);
}
