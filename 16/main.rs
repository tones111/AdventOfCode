use std::io::{self, BufRead};

// struct Reindeer {
//    name: String,
//    speed: u32,
//    fly_time: u32,
//    rest_time: u32,
//    dist: u64,
//    points: u32,
// }

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let tokens: Vec<_> = line.split_whitespace().collect();

        println!("tokens: {:?}", tokens);
    }
}
