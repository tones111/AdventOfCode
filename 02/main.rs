use std::io;

fn main() {
    let mut area = 0;
    let mut length = 0;

    let mut line = String::new();
    loop {
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        if line.len() == 0 {
                break;
        }
        let dims: Vec<_> = line.trim_right().split('x')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let l = dims[0];
        let w = dims[1];
        let h = dims[2];

        area += 2 * l * w + 2 * l * h + 2 * w * h + slack(l, w, h);
        length += ribbon(l, w, h);
    }
    println!("area: {}", area);
    println!("length: {}", length);
}

fn min(a: u32, b: u32) -> u32 {
    if a < b {
        a
    } else {
        b
    }
}

fn short_sides(l: u32, w: u32, h: u32) -> (u32, u32) {
    if l < w {
        (l, min(w, h))
    } else {
        (w, min(l, h))
    }
}

fn slack(l: u32, w: u32, h: u32) -> u32 {
    let (s1, s2) = short_sides(l, w, h);
    s1 * s2
}

fn ribbon(l: u32, w: u32, h: u32) -> u32 {
    let (s1, s2) = short_sides(l, w, h);
    2 * s1 + 2 * s2 + l * w * h
}
