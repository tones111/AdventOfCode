extern crate crypto;

use std::io::{self, Read};
use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    let mut key = String::new();
    io::stdin().read_to_string(&mut key).unwrap();
    let key = key.trim_right().as_bytes();

    let mut digest = Md5::new();
    let mut output = [0u8; 16];

    let mut passed5 = false;
    for i in 1u64.. {
        digest.reset();
        digest.input(key);
        digest.input(format!("{}", i).as_bytes());
        digest.result(&mut output);

        let mut passed = true;
        for j in 0..2 {
            if output[j] != 0 {
                passed = false;
                break;
            }
        }
        if passed && !passed5 && (output[2] & 0xF0) == 0 {
            print!("{}: ", i);
            print_debug(&output);
            println!("");
            passed5 = true;
        }
        if passed && (output[2] & 0xFF) == 0 {
            print!("{}: ", i);
            print_debug(&output);
            println!("");
            break;
        }
    }
}

fn print_debug(s: &[u8]) {
    for b in s.iter() {
        print!("{:02x}", b);
    }
}
