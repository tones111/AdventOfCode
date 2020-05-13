use std::collections::HashSet;
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let deltas = input
        .lines()
        .map(|l| l.parse::<i32>())
        .collect::<std::result::Result<Vec<_>, _>>()?;

    // Part 1
    println!("delta: {}", deltas.iter().fold(0i32, |acc, x| acc + x));

    // Part 2
    let mut freqs = HashSet::new();
    let mut freq = 0;
    freqs.insert(freq);

    'outer: loop {
        for delta in deltas.iter() {
            freq += delta;
            if !freqs.insert(freq) {
                break 'outer;
            }
        }
    }
    println!("freq: {}", freq);

    Ok(())
}
