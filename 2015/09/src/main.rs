use std::collections::HashMap;
use std::io::{self, BufRead};

extern crate permutohedron;
use permutohedron::Heap;

fn main() {
    // Parse routes, map cities to unique indices, build map
    let mut map;
    let mut columns = HashMap::new();
    {
        let mut routes = Vec::new();
        let mut index = 0;
        let stdin = io::stdin();
        for line in stdin.lock().lines().map(|l| l.unwrap()) {
            let tokens: Vec<_> = line.split_whitespace().collect();

            let mut indices = Vec::new();
            for city in &[tokens[0], tokens[2]] {
                indices.push(*columns.entry(city.to_string()).or_insert_with(|| {
                    let i = index;
                    index += 1;
                    i
                }));
            }
            routes.push((indices, tokens[4].parse::<u32>().unwrap()));

        }
        let routes = routes;
        // println!("{:?}", routes);

        // Build map from route information
        map = vec![vec![None; columns.len()]; columns.len()];
        for route in routes {
            // Assume distances are bi-directional
            map[route.0[0]][route.0[1]] = Some(route.1);
            map[route.0[1]][route.0[0]] = Some(route.1);
        }
    }
    let map = map;
    let columns = columns;
    println!("{:?}", columns);

    let mut shortest_path = None;
    let mut shortest_dist = None;
    let mut longest_path = None;
    let mut longest_dist = None;

    let mut city_ids: Vec<_> = (0..columns.len()).collect();
    for path in Heap::new(&mut city_ids) {
        let mut path_iter = path.iter();
        let mut cur = path_iter.next().unwrap();

        if let Ok(dists) = path_iter.map(|next| {
                let dist = match map[*cur][*next] {
                    Some(d) => Ok(d),
                    None => Err(()),
                };
                cur = next;
                dist
            })
            .collect::<Result<Vec<_>, _>>() {

            let dist = dists.into_iter().fold(0, |total, edge| total + edge);
            shortest_dist = match shortest_dist {
                None => {
                    shortest_path = Some(path.clone());
                    Some(dist)
                }
                Some(shortest) if dist < shortest => {
                    shortest_path = Some(path.clone());
                    Some(dist)
                }
                _ => shortest_dist,
            };

            longest_dist = match longest_dist {
                None => {
                    longest_path = Some(path.clone());
                    Some(dist)
                }
                Some(longest) if dist > longest => {
                    longest_path = Some(path.clone());
                    Some(dist)
                }
                _ => longest_dist,
            };
        };
    }

    println!("Shortest Path: {:?} => {}",
             shortest_path.unwrap(),
             shortest_dist.unwrap());
    println!("Longest Path: {:?} => {}",
             longest_path.unwrap(),
             longest_dist.unwrap());
}
