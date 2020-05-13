use std::io::{self, Read};

fn main() {
    let mut say = Vec::new();
    for b in io::stdin().bytes().map(|b| b.unwrap()).filter(|b| 0x30 <= *b && *b <= 0x39) {
        say.push(b - 0x30);
    }
    // println!("0: {:?}", say);

    for round in 1..51 {
        let mut next: Vec<u8> = Vec::with_capacity(say.len());
        let mut last = None;
        let mut count = 0;

        for i in say.into_iter() {
            if last == None || last == Some(i) {
                count += 1;
            } else {
                next.push(count);
                next.push(last.unwrap());
                count = 1;
            }
            last = Some(i);
        }
        next.push(count);
        next.push(last.unwrap());
        say = next;

        // println!("{}: {:?}", round, say);

        if round == 40 || round == 50 {
            println!("length: {}", say.len());
        }
    }
}
