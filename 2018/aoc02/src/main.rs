use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let ids = input.lines().collect::<Vec<_>>();

    // Part 1
    let mut id2_qty = 0i32;
    let mut id3_qty = 0i32;
    for id in ids.iter() {
        // Note: This method assumes ASCII input
        let mut freqs = [0u8; u8::MAX as usize + 1];
        for f in freqs.iter_mut() {
            *f = 0;
        }

        for b in id.as_bytes().iter().map(|&b| b as usize) {
            freqs[b] = freqs[b].saturating_add(1);
        }

        if freqs.iter().any(|&f| f == 2) {
            id2_qty += 1;
        }
        if freqs.iter().any(|&f| f == 3) {
            id3_qty += 1;
        }
    }
    println!("hash: {} * {} = {}", id2_qty, id3_qty, id2_qty * id3_qty);

    // Part 2
    for i in 0..ids.len() {
        for j in i + 1..ids.len() {
            let id1 = ids[i];
            let id2 = ids[j];

            if id1.len() == id2.len()
                && id1
                    .chars()
                    .zip(id2.chars())
                    .filter(|c| c.0 != c.1)
                    .take(2)
                    .count()
                    <= 1
            {
                println!("{} ~= {}", id1, id2);
                println!(
                    "common: {}",
                    id1.chars()
                        .zip(id2.chars())
                        .filter(|&(c1, c2)| c1 == c2)
                        .map(|(c, _)| c)
                        .collect::<String>()
                );
            }
        }
    }

    Ok(())
}
