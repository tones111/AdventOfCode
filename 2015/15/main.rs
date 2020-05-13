fn main() {
    // let props = [[-1, -2, 6, 3, 8], [2, 3, -2, -1, 3], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0]];
    let props = [[4, -2, 0, 0, 5], [0, 5, -1, 0, 8], [-1, 0, 5, 0, 6], [0, 0, -2, 2, 1]];

    let mut max = None;
    let mut max_cal = None;

    for i in 0..101 {
        for j in 0..101 - i {
            for k in 0..101 - i - j {
                let l = 100 - i - j - k;

                let c = std::cmp::max(0,
                                      i * props[0][0] + j * props[1][0] + k * props[2][0] +
                                      l * props[3][0]);
                let d = std::cmp::max(0,
                                      i * props[0][1] + j * props[1][1] + k * props[2][1] +
                                      l * props[3][1]);
                let f = std::cmp::max(0,
                                      i * props[0][2] + j * props[1][2] + k * props[2][2] +
                                      l * props[3][2]);
                let t = std::cmp::max(0,
                                      i * props[0][3] + j * props[1][3] + k * props[2][3] +
                                      l * props[3][3]);
                let score = c * d * f * t;

                match max {
                    None => max = Some(score),
                    Some(m) if score > m => {
                        max = Some(score);
                        // println!("{}, {}, {}, {} => {}", i, j, k, l, score);
                    }
                    _ => {}
                }

                let cal = std::cmp::max(0,
                                        i * props[0][4] + j * props[1][4] + k * props[2][4] +
                                        l * props[3][4]);
                if cal != 500 {
                    continue;
                }

                match max_cal {
                    None => max_cal = Some(score),
                    Some(m) if score > m => {
                        max_cal = Some(score);
                        // println!("{}, {}, {}, {} => {}", i, j, k, l, score);
                    }
                    _ => {}
                }

            }
        }
    }
    println!("Part 1: {}", max.unwrap());
    println!("Part 2: {}", max_cal.unwrap());
}
