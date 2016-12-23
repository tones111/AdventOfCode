use std::collections::HashMap;
use std::io::{self, BufRead};

extern crate permutohedron;
use permutohedron::Heap;

fn main() {
    // Parse seating preferences, map people to unique indices, build table
    let mut table;
    let mut columns = HashMap::new();
    {
        let mut scores = Vec::new();
        let mut index = 0;
        let stdin = io::stdin();
        for line in stdin.lock().lines().map(|l| l.unwrap()) {
            let tokens: Vec<_> = line.split_whitespace().collect();

            let mut indices = Vec::new();
            for person in &[tokens[0], tokens[10].trim_right_matches('.')] {
                indices.push(*columns.entry(person.to_string()).or_insert_with(|| {
                    let i = index;
                    index += 1;
                    i
                }));
            }
            scores.push((indices,
                         match tokens[2] {
                             "gain" => 1,
                             "lose" => -1,
                             _ => panic!("unexpected happines modifier"),
                         } * tokens[3].parse::<u32>().unwrap() as i32));

        }
        let scores = scores;

        // Build table from happiness score information
        table = vec![vec![None; columns.len()]; columns.len()];
        for pair in scores {
            table[pair.0[0]][pair.0[1]] = Some(pair.1);
        }
    }

    let score_without = most_happy(&table);
    println!("Optimal Happness: {}", score_without);

    // Add myself as a new column in the table
    for row in &mut table {
        row.push(Some(0));
    }

    // Add now for my row
    let mut me = Vec::new();
    for i in 0..table[0].len() {
        if i == table[0].len() - 1 {
            me.push(None);
        } else {
            me.push(Some(0));
        }
    }
    table.push(me);

    let score_with = most_happy(&table);
    println!("with me: {}", score_with);
}

fn most_happy(table: &[Vec<Option<i32>>]) -> i32 {
    let num_people = table.len();
    let mut high_score = None;

    let mut per_ids: Vec<_> = (0..num_people).collect();
    for order in Heap::new(&mut per_ids) {
        let mut score = 0;
        for (i, cur) in order.iter().enumerate() {
            if i == 0 {
                let prev = order[num_people - 1];
                let next = order[i + 1];
                score += table[*cur][prev].unwrap();
                score += table[*cur][next].unwrap();
            } else if i == num_people - 1 {
                let prev = order[i - 1];
                let next = order[0];
                score += table[*cur][prev].unwrap();
                score += table[*cur][next].unwrap();
            } else {
                let prev = order[i - 1];
                let next = order[i + 1];
                score += table[*cur][prev].unwrap();
                score += table[*cur][next].unwrap();
            }
        }

        high_score = match high_score {
            None => Some(score),
            Some(high) if score > high => Some(score),
            _ => high_score,
        };
    }
    high_score.unwrap()
}
