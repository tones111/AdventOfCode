use std::io;
use std::str::FromStr;

#[derive(Debug)]
enum Command {
    Toggle,
    Off,
    On,
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

#[derive(Debug)]
struct Action {
    start: Point,
    end: Point,
    cmd: Command,
}

impl FromStr for Action {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut action_index = 0;
        let mut start_index = 0;
        let mut end_index = 0;

        let s = s.trim().split(' ').collect::<Vec<_>>();
        match s[0] {
            "toggle" => {
                action_index = 0;
                start_index = 1;
                end_index = 3;
            }
            "turn" => {
                action_index = 1;
                start_index = 2;
                end_index = 4;
            }
            _ => {
                return Err(());
            }
        };
        Ok(Action {
            cmd: s[action_index].parse().unwrap(),
            start: {
                let mut p = s[start_index].split(',');
                Point {
                    x: p.next().unwrap().parse().unwrap(),
                    y: p.next().unwrap().parse().unwrap(),
                }
            },
            end: {
                let mut p = s[end_index].split(',');
                Point {
                    x: p.next().unwrap().parse().unwrap(),
                    y: p.next().unwrap().parse().unwrap(),
                }
            },
        })
    }
}

fn main() {
    let mut lights = [[false; 1000]; 1000];
    let mut brights = [[0u8; 1000]; 1000];

    let mut line = String::new();
    loop {
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        let line = line.trim_right();

        if line.len() == 0 {
            break;
        }

        let action: Action = line.parse().unwrap();
        // println!("{:?}", action);
        for i in action.start.x..action.end.x + 1 {
            for j in action.start.y..action.end.y + 1 {
                match action.cmd {
                    Command::Toggle => {
                        lights[i][j] ^= lights[i][j];
                    }
                    Command::On => {
                        lights[i][j] = true;
                    }
                    Command::Off => {
                        lights[i][j] = false;
                    }
                }
            }
        }

        for i in action.start.x..action.end.x + 1 {
            for j in action.start.y..action.end.y + 1 {
                match action.cmd {
                    Command::Toggle => {
                        brights[i][j] += 2;
                    }
                    Command::On => {
                        brights[i][j] += 1;
                    }
                    Command::Off => {
                        if brights[i][j] > 0 {
                            brights[i][j] -= 1;
                        }
                    }
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
