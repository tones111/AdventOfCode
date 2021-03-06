use std::io::{self, BufRead};

fn main() {
    let mut num_nice1 = 0;
    let mut num_nice2 = 0;

    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        if nice1(&line) {
            num_nice1 += 1;
        }
        if nice2(&line) {
            num_nice2 += 1;
        }
    }
    println!("{} nice words (1)", num_nice1);
    println!("{} nice words (2)", num_nice2);
}

fn nice1(word: &str) -> bool {
    for bad in ["ab", "cd", "pq", "xy"].iter() {
        if word.contains(bad) {
            return false;
        }
    }

    let mut vowels = 0;
    let mut double = false;

    let mut prev: char = 0 as char;
    for (i, c) in word.chars().enumerate() {
        vowels += match c {
            'a' | 'e' | 'i' | 'o' | 'u' => 1,
            _ => 0,
        };

        if i != 0 && !double {
            double = c == prev;
        }
        prev = c;
    }
    vowels >= 3 && double
}

fn nice2(word: &str) -> bool {
    let length = word.len();

    let mut double = false;
    let mut same_char = false;

    let mut prev1: char = 0 as char;
    let mut prev2: char = 0 as char;
    for (i, c) in word.chars().enumerate() {
        if i >= 2 {
            if !double {
                // Note: This doesn't work for multi-byte chars
                double = (&word[i..length]).contains(&word[i - 2..i]);
            }
            if !same_char {
                same_char = c == prev2;
            }
        }
        if double && same_char {
            break;
        }

        prev2 = prev1;
        prev1 = c;
    }
    double && same_char
}
