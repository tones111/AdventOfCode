use std::io::{self, Read};
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

fn main() {
    // puzzle 1
    let mut pos = Coord { x: 0, y: 0 };
    let mut visited = HashSet::new();
    visited.insert(pos);

    // puzzle 2
    let mut pos1 = Coord { x: 0, y: 0 };
    let mut pos2 = Coord { x: 0, y: 0 };
    let mut visited2 = HashSet::new();
    visited2.insert(pos1);

    for (i, dir) in io::stdin().bytes().enumerate() {
        let dir = dir.unwrap() as char;
        // puzzle 1
        pos = move_pos(pos, dir);
        visited.insert(pos);

        // puzzle 2
        if i % 2 == 0 {
            pos1 = move_pos(pos1, dir);
            visited2.insert(pos1);
        } else {
            pos2 = move_pos(pos2, dir);
            visited2.insert(pos2);
        }
    }
    println!("visited: {}", visited.len());
    println!("visited2: {}", visited2.len());
}

fn move_pos(c: Coord, dir: char) -> Coord {
    let mut pos = c;
    match dir {
        '^' => {
            pos.y += 1;
        }
        '>' => {
            pos.x += 1;
        }
        'v' => {
            pos.y -= 1;
        }
        '<' => {
            pos.x -= 1;
        }
        _ => {}
    }
    pos
}
