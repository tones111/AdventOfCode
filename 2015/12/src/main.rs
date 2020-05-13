use std::io::{self, Read};

extern crate serde_json;

use serde_json::Value;

fn main() {

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let data: Value = serde_json::from_str(&buffer).unwrap();
    println!("all: {}", sum(&data));
    println!("no red: {}", sum_nored(&data));
}

fn sum(v: &Value) -> i64 {
    match *v {
        Value::I64(i) => i,
        Value::U64(u) => u as i64,
        Value::Array(ref a) => a.iter().map(|v| sum(v)).sum(),
        Value::Object(ref m) => m.values().map(|v| sum(v)).sum(),

        Value::Null => {
            println!("Null");
            0
        }
        Value::Bool(_) => {
            println!("Bool");
            0
        }
        Value::F64(_) => {
            println!("F64");
            0
        }
        Value::String(_) => 0,
    }
}

fn sum_nored(v: &Value) -> i64 {
    match *v {
        Value::I64(i) => i,
        Value::U64(u) => u as i64,
        Value::Array(ref a) => a.iter().map(|v| sum_nored(v)).sum(),
        Value::Object(ref m) => {
            let has_red = m.values()
                .filter(|v| {
                    match **v {
                        Value::String(ref s) if s == "red" => true,
                        _ => false,
                    }
                })
                .count() > 0;

            if has_red {
                0
            } else {
                m.values().map(|v| sum_nored(v)).sum()
            }
        }
        Value::Null => {
            println!("Null");
            0
        }
        Value::Bool(_) => {
            println!("Bool");
            0
        }
        Value::F64(_) => {
            println!("F64");
            0
        }
        Value::String(_) => 0,
    }
}
