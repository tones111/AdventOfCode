use std::io::{self, BufRead};

// todo: use unit types for improved compile-time checks and to learn how to implement conversions cleanly

struct Reindeer {
    name: String,
    speed: u32,
    fly_time: u32,
    rest_time: u32,
    dist: u64,
    points: u32,
}

fn main() {
    let mut race = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let tokens: Vec<_> = line.split_whitespace().collect();

        race.push(Reindeer {
            name: tokens[0].to_string(),
            speed: tokens[3].parse::<u32>().unwrap(),
            fly_time: tokens[6].parse::<u32>().unwrap(),
            rest_time: tokens[13].parse::<u32>().unwrap(),
            dist: 0,
            points: 0,
        });
    }

    let time = 2503; // seconds
    for tick in 1..time + 1 {
        for rdeer in race.iter_mut() {
            let period = rdeer.fly_time + rdeer.rest_time;
            let flying = tick / period * rdeer.fly_time +
                         std::cmp::min(tick % period, rdeer.fly_time);
            rdeer.dist = flying as u64 * rdeer.speed as u64;
        }

        // award points per frame
        let lead_dist = race.iter().map(|r| r.dist).max().unwrap();
        for rdeer in race.iter_mut().filter(|r| r.dist == lead_dist) {
            rdeer.points += 1;
        }
    }

    println!("Results:");
    for rdeer in race.iter() {
        println!("{} ({}km) => {}pts", rdeer.name, rdeer.dist, rdeer.points);
    }
    println!("");

    println!("winner (dist):   {}",
             race.iter().map(|r| r.dist).max().unwrap());
    println!("winner (points): {}",
             race.iter().map(|r| r.points).max().unwrap());
}
