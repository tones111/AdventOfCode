use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read};
use std::str::FromStr;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;
fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    // TODO: include line number in error message
    let claims = input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<Claim>>>()?;

    let mut allocated: HashMap<Point, u32> = HashMap::new();
    for claim in &claims {
        for p in claim.iter_points() {
            *allocated.entry(p).or_default() += 1;
        }
    }

    println!(
        "overlap: {}",
        allocated.values().filter(|&&c| c > 1).count()
    );

    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Size {
    w: u32,
    h: u32,
}

#[derive(Debug)]
struct Claim {
    id: u32,
    pos: Point,
    size: Size,
}

impl Claim {
    fn iter_points(&self) -> IterPoints {
        IterPoints {
            claim: self,
            p: self.pos,
        }
    }
}

struct IterPoints<'c> {
    claim: &'c Claim,
    p: Point,
}

impl<'c> Iterator for IterPoints<'c> {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        if self.p.x >= self.claim.pos.x + self.claim.size.w {
            self.p.x = self.claim.pos.x;
            self.p.y += 1;
        }
        if self.p.y >= self.claim.pos.y + self.claim.size.h {
            return None;
        }
        self.p.x += 1;
        Some(Point {
            x: self.p.x - 1,
            y: self.p.y,
        })
    }
}

impl FromStr for Claim {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        use lazy_static::lazy_static;
        use regex::Regex;

        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
            \#(?P<id>[0-9]+)
            \s+@\s+
            (?P<x>[0-9]+),(?P<y>[0-9]+):
            \s+
            (?P<w>[0-9]+)x(?P<h>[0-9]+)"
            )
            .unwrap();
        }

        let caps = RE.captures(s).ok_or("unrecognized claim")?;
        Ok(Claim {
            id: caps["id"].parse()?,
            pos: Point {
                x: caps["x"].parse()?,
                y: caps["y"].parse()?,
            },
            size: Size {
                w: caps["w"].parse()?,
                h: caps["h"].parse()?,
            },
        })
    }
}
