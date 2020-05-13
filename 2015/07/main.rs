use std::collections::HashMap;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct WireId(String);

#[derive(Debug, Eq, Hash, PartialEq)]
enum Signal {
    Wire(WireId),
    Const(u16),
}

impl FromStr for Signal {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(val) = s.parse::<u16>() {
            Ok(Signal::Const(val))
        } else {
            Ok(Signal::Wire(WireId(s.to_string())))
        }
    }
}

#[derive(Debug)]
enum Gate {
    Wire(Signal),
    Not(Signal),
    Lshift(Signal, u8),
    Rshift(Signal, u8),
    And(Signal, Signal),
    Or(Signal, Signal),
}

impl FromStr for Gate {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let tok1 = iter.next().unwrap();

        match tok1 {
            // Prefix operations
            "NOT" => Ok(Gate::Not(iter.next().unwrap().parse::<Signal>().unwrap())),

            _ => {
                match iter.next().unwrap() {
                    // Constant or wire
                    "->" => Ok(Gate::Wire(tok1.parse::<Signal>().unwrap())),

                    // Binary operations
                    "AND" => {
                        Ok(Gate::And(tok1.parse::<Signal>().unwrap(),
                                     iter.next().unwrap().parse::<Signal>().unwrap()))
                    }

                    "OR" => {
                        Ok(Gate::Or(tok1.parse::<Signal>().unwrap(),
                                    iter.next().unwrap().parse::<Signal>().unwrap()))
                    }

                    "RSHIFT" => {
                        Ok(Gate::Rshift(tok1.parse::<Signal>().unwrap(),
                                        iter.next().unwrap().parse::<u8>().unwrap()))
                    }

                    "LSHIFT" => {
                        Ok(Gate::Lshift(tok1.parse::<Signal>().unwrap(),
                                        iter.next().unwrap().parse::<u8>().unwrap()))
                    }
                    _ => {
                        println!("Unexpected instruction: {}", s);
                        Err(())
                    }
                }
            }
        }
    }
}

fn main() {
    let mut gates = HashMap::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let gate: Gate = line.parse().unwrap();
        let dest = WireId(line.split_whitespace().last().unwrap().to_string());

        // println!("{:?}", gate);
        gates.insert(dest, gate);
    }
    let gates = gates;

    let mut wires = HashMap::new();
    let wire_a = evaluate(&Signal::Wire(WireId("a".to_string())), &gates, &mut wires);
    println!("Part 1: a = {}", wire_a);

    wires.clear();
    wires.insert(WireId("b".to_string()), wire_a);

    let wire_a = evaluate(&Signal::Wire(WireId("a".to_string())), &gates, &mut wires);
    println!("Part 2: a = {}", wire_a);
}

fn evaluate(sig: &Signal, gates: &HashMap<WireId, Gate>, wires: &mut HashMap<WireId, u16>) -> u16 {
    match sig {
        &Signal::Const(v) => v,
        &Signal::Wire(ref id) => {
            // Check cache first
            if let Some(v) = wires.get(id) {
                return *v;
            }

            // Otherwise, compute
            let v = match gates.get(id).unwrap() {
                &Gate::Wire(ref s) => evaluate(s, gates, wires),
                &Gate::Not(ref s) => !evaluate(s, gates, wires),

                &Gate::Lshift(ref s, b) => evaluate(s, gates, wires) << b,
                &Gate::Rshift(ref s, b) => evaluate(s, gates, wires) >> b,

                &Gate::And(ref ls, ref rs) => {
                    evaluate(ls, gates, wires) & evaluate(rs, gates, wires)
                }
                &Gate::Or(ref ls, ref rs) => {
                    evaluate(ls, gates, wires) | evaluate(rs, gates, wires)
                }
            };
            // println!("{:?} = {}", id, v);
            wires.insert((*id).clone(), v);
            v
        }
    }
}
