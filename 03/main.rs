use std::io::{self, Read};
use std::collections::HashSet;

#[derive(Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn mv(&mut self, d: Direction) {
        match d {
            Direction::North => self.y += 1,
            Direction::East => self.x += 1,
            Direction::South => self.y -= 1,
            Direction::West => self.x -= 1,
        }
    }
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

    for (i, dir) in io::stdin()
        .bytes()
        .filter_map(|d| {
            match d.unwrap() as char {
                '^' => Some(Direction::North),
                '>' => Some(Direction::East),
                'v' => Some(Direction::South),
                '<' => Some(Direction::West),
                _ => None,
            }
        })
        .enumerate() {

        // puzzle 1
        pos.mv(dir);
        visited.insert(pos);

        // puzzle 2
        if i % 2 == 0 {
            pos1.mv(dir);
            visited2.insert(pos1);
        } else {
            pos2.mv(dir);
            visited2.insert(pos2);
        }
    }
    println!("visited: {}", visited.len());
    println!("visited2: {}", visited2.len());
}
