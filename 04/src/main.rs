extern crate crypto;

use std::fmt::Write;
use std::io::{self, Read};
use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    let mut key = String::new();
    io::stdin().read_to_string(&mut key).unwrap();
    let key = key.trim_right().as_bytes();

    let mut digest = Md5::new();
    let mut output = [0u8; 16];

    let leading0_1 = 5;
    let mut found_1 = false;
    let leading0_2 = 6;
    let mut found_2 = false;

    for i in 1u64.. {
        digest.reset();
        digest.input(key);
        digest.input(format!("{}", i).as_bytes());
        digest.result(&mut output[..]);

        if test_hash(&output, leading0_1) && !found_1 {
            found_1 = true;
            println!("{}: {} {}", leading0_1, i, hash2string(&output));
        }
        if test_hash(&output, leading0_2) && !found_2 {
            found_2 = true;
            println!("{}: {} {}", leading0_2, i, hash2string(&output));
        }

        if found_1 && found_2 {
            break;
        }
    }
}

fn test_hash(hash: &[u8], num_zeroes: u8) -> bool {
    let bytes = num_zeroes / 2;

    if num_zeroes == 0 {
        return true;
    } else if num_zeroes >= 2 {
        for i in 0..bytes {
            if hash[i as usize] != 0 {
                return false;
            }
        }
    }
    num_zeroes % 2 == 0 || hash[bytes as usize] & 0xF0 == 0
}

fn hash2string(hash: &[u8]) -> String {
    let mut s = String::new();
    for b in hash.iter() {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}
