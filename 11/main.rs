use std::io::{self, Read};

fn main() {
    let mut pass = Vec::new();
    for b in io::stdin().bytes().map(|b| b.unwrap()).filter(|b| 0x61 <= *b && *b <= 0x7a) {
        pass.push(b as char);
    }
    println!("original: {:?}", pass);

    for _ in 0..2 {
        loop {
            inc(&mut pass[..]);
            if valid(&pass[..]) {
                break;
            }
            // println!("{:?} -> {}", pass, valid(&pass[..]));
        }
        println!("{:?}", pass);
    }
}

fn valid(s: &[char]) -> bool {
    let mut valid_chars = true;

    let mut found_straight = false;
    let mut straight_len = 1;

    let mut double_cnt = 0;

    let mut last_straight: Option<&char> = None;
    let mut last_double: Option<&char> = None;
    for c in s.iter() {
        match *c {
            'i' | 'o' | 'l' => {
                valid_chars = false;
                break;
            }
            _ => {}
        }

        match last_straight {
            Some(prev) if *c == (*prev as u8 + 1) as char => {
                straight_len += 1;
                if straight_len == 3 {
                    found_straight = true;
                }
            }
            _ => straight_len = 1,
        }
        last_straight = Some(c);

        match last_double {
            Some(prev) if *c == *prev => {
                double_cnt += 1;
                last_double = None;
            }
            _ => last_double = Some(c),
        }
    }
    found_straight && valid_chars && double_cnt >= 2
}

fn inc(s: &mut [char]) {
    for i in (0..s.len()).rev() {
        s[i] = match s[i] {
            'z' => 'a',
            c => (c as u8 + 1) as char,
        };
        if s[i] != 'a' {
            break;
        }
    }
}
