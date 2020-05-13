use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug)]
enum Command {
    Off,
    On,
    Toggle,
}

impl FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Command::*;
        match s {
            "toggle" => Ok(Toggle),
            "on" => Ok(On),
            "off" => Ok(Off),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(',').collect();
        Ok(Point {
            x: try!(coords[0].parse()),
            y: try!(coords[1].parse()),
        })
    }
}

#[derive(Debug)]
struct Action {
    start: Point,
    end: Point,
    cmd: Command,
}

impl FromStr for Action {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = Point { x: 0, y: 0 };
        let mut end = Point { x: 0, y: 0 };

        for (i, tok) in s.split(' ').rev().enumerate() {
            // println!("{}: {}", i, tok);
            match i {
                0 => end = tok.parse().unwrap(),
                2 => start = tok.parse().unwrap(),
                3 => {
                    return Ok(Action {
                        cmd: tok.parse().unwrap(),
                        start: start,
                        end: end,
                    })
                }
                _ => {}
            }
        }
        Err(())
    }
}

fn main() {
    let mut lights = [[false; 1000]; 1000]; // puzzle 1
    let mut brights = [[0u8; 1000]; 1000];  // puzzle 2

    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {

        let action: Action = line.parse().unwrap();
        // println!("{:?}", action);

        for i in action.start.x..action.end.x + 1 {
            for j in action.start.y..action.end.y + 1 {
                // Puzzle 1
                lights[i][j] = match action.cmd {
                    Command::On => true,
                    Command::Off => false,
                    Command::Toggle => !lights[i][j],
                };

                // Puzzle 2
                match action.cmd {
                    Command::On => brights[i][j] += 1,
                    Command::Off if brights[i][j] > 0 => brights[i][j] -= 1,
                    Command::Off => {}
                    Command::Toggle => brights[i][j] += 2,
                }
            }
        }
    }

    let mut on_cnt = 0usize;
    let mut brightness = 0u64;

    for i in 0..1000 {
        for j in 0..1000 {
            if lights[i][j] {
                on_cnt += 1;
            }
            brightness += brights[i][j] as u64;
        }
    }
    println!("{} lights on", on_cnt);
    println!("{} brightness", brightness);
}
