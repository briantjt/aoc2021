use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Cave<'a> {
    Start,
    Small(&'a str),
    Big(&'a str),
    End,
}

impl<'a> Cave<'a> {
    fn new(s: &'a str) -> Self {
        if s.chars().all(char::is_uppercase) {
            Big(s)
        } else {
            Small(s)
        }
    }
}
use Cave::*;

fn explore(cave_map: &BTreeMap<Cave, Vec<Cave>>, can_visit_twice: bool) -> u64 {
    let mut num_paths = 0;
    let mut visited: BTreeSet<Cave> = BTreeSet::new();
    let mut to_explore_paths = vec![(Start, 0)];
    let mut cave_visited_twice = None;
    while !to_explore_paths.is_empty() {
        let mut current = to_explore_paths.last_mut().unwrap();
        let neighbors = cave_map.get(&current.0).unwrap();
        if current.0 == End || current.1 == neighbors.len() {
            // Reached the end or explored all neighbors
            if current.0 == End {
                num_paths += 1;
            }
            if cave_visited_twice == Some(current.0) {
                cave_visited_twice = None
            } else {
                visited.remove(&current.0);
            }
            to_explore_paths.pop();
        } else {
            let next = neighbors[current.1];
            current.1 += 1;
            if let Small(_) = next {
                let has_visited = visited.contains(&next);
                let cannot_visit_again = !can_visit_twice || cave_visited_twice.is_some();
                if has_visited {
                    if cannot_visit_again {
                        continue;
                    }
                    cave_visited_twice = Some(next);
                }
                visited.insert(next);
            }
            to_explore_paths.push((next, 0));
        }
    }
    num_paths
}

fn main() -> std::io::Result<()> {
    let contents = include_str!("input.txt");
    let mut cave_map = BTreeMap::new();
    cave_map.insert(Start, Vec::new());
    cave_map.insert(End, Vec::new());
    for line in contents.lines() {
        let parts: Vec<&str> = line.split('-').collect();
        if parts[0] == "start" || parts[1] == "start" {
            let cave = if parts[0] == "start" {
                Cave::new(parts[1])
            } else {
                Cave::new(parts[0])
            };
            cave_map.get_mut(&Start).unwrap().push(cave);
        } else if parts[0] == "end" || parts[1] == "end" {
            let cave = if parts[0] == "end" {
                Cave::new(parts[1])
            } else {
                Cave::new(parts[0])
            };
            cave_map.entry(cave).or_insert_with(Vec::new).push(End);
        } else {
            let cave1 = Cave::new(parts[0]);
            let cave2 = Cave::new(parts[1]);
            cave_map.entry(cave1).or_insert_with(Vec::new).push(cave2);
            cave_map.entry(cave2).or_insert_with(Vec::new).push(cave1);
        }
    }
    let num_paths = explore(&cave_map, false);
    println!("{}", num_paths);
    let num_paths = explore(&cave_map, true);
    println!("{}", num_paths);

    Ok(())
}
