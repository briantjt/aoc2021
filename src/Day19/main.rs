use std::collections::{HashMap, HashSet};

use itertools::Itertools;
#[macro_use]
extern crate scan_fmt;

use nalgebra::{Matrix3, Vector3};

static ROTATIONS: [Matrix3<i32>; 24] = [
    Matrix3::new(1, 0, 0, 0, 1, 0, 0, 0, 1),
    Matrix3::new(1, 0, 0, 0, 0, 1, 0, -1, 0),
    Matrix3::new(1, 0, 0, 0, -1, 0, 0, 0, -1),
    Matrix3::new(1, 0, 0, 0, 0, -1, 0, 1, 0),
    Matrix3::new(0, 1, 0, 0, 0, 1, 1, 0, 0),
    Matrix3::new(0, 1, 0, 1, 0, 0, 0, 0, -1),
    Matrix3::new(0, 1, 0, 0, 0, -1, -1, 0, 0),
    Matrix3::new(0, 1, 0, -1, 0, 0, 0, 0, 1),
    Matrix3::new(0, 0, 1, 1, 0, 0, 0, 1, 0),
    Matrix3::new(0, 0, 1, 0, 1, 0, -1, 0, 0),
    Matrix3::new(0, 0, 1, -1, 0, 0, 0, -1, 0),
    Matrix3::new(0, 0, 1, 0, -1, 0, 1, 0, 0),
    Matrix3::new(-1, 0, 0, 0, -1, 0, 0, 0, 1),
    Matrix3::new(-1, 0, 0, 0, 0, 1, 0, 1, 0),
    Matrix3::new(-1, 0, 0, 0, 1, 0, 0, 0, -1),
    Matrix3::new(-1, 0, 0, 0, 0, -1, 0, -1, 0),
    Matrix3::new(0, -1, 0, 0, 0, -1, 1, 0, 0),
    Matrix3::new(0, -1, 0, 1, 0, 0, 0, 0, 1),
    Matrix3::new(0, -1, 0, 0, 0, 1, -1, 0, 0),
    Matrix3::new(0, -1, 0, -1, 0, 0, 0, 0, -1),
    Matrix3::new(0, 0, -1, -1, 0, 0, 0, 1, 0),
    Matrix3::new(0, 0, -1, 0, 1, 0, 1, 0, 0),
    Matrix3::new(0, 0, -1, 1, 0, 0, 0, -1, 0),
    Matrix3::new(0, 0, -1, 0, -1, 0, -1, 0, 0),
];

type Vector = Vector3<i32>;
type Pair = (Vector, Vector);
type Tag = (i32, Pair);

fn main() -> std::io::Result<()> {
    let contents = include_str!("input.txt");
    let mut beacon_lists: Vec<Vec<_>> = contents
        .split("\n\n")
        .map(|section| {
            section
                .lines()
                .skip(1)
                .map(|line| {
                    let (x, y, z) = scan_fmt!(line, "{},{},{}", i32, i32, i32).unwrap();
                    Vector3::new(x, y, z)
                })
                .collect()
        })
        .collect();

    let first = beacon_lists.swap_remove(0);
    let mut dist_to_beacons: HashMap<i32, HashSet<(Vector, Vector)>> = HashMap::new();
    let mut normalized_beacons: HashSet<Vector> = HashSet::new();
    first.into_iter().tuple_combinations().for_each(|(b1, b2)| {
        let relative_vec = b2 - b1;
        normalized_beacons.insert(b1);
        normalized_beacons.insert(b2);
        dist_to_beacons
            .entry(relative_vec[0].abs() + relative_vec[1].abs() + relative_vec[2].abs())
            .or_default()
            .insert((b1, b2));
    });
    let mut scanners = vec![Vector3::new(0, 0, 0)];
    let mut processed_lists: Vec<(Vec<Vector>, Vec<Tag>)> = beacon_lists
        .into_iter()
        .map(|list| {
            let relative_vecs: Vec<(i32, Pair)> = list
                .iter()
                .tuple_combinations()
                .map(|(&b1, &b2)| {
                    let vec = b2 - b1;
                    ((vec[0].abs() + vec[1].abs() + vec[2].abs()), (b1, b2))
                })
                .collect();
            (list, relative_vecs)
        })
        .collect();
    while !processed_lists.is_empty() {
        let mut to_remove = None;
        'outer: for (idx, (list, tags)) in processed_lists.iter().enumerate() {
            let common: Vec<_> = tags
                .iter()
                .filter(|(f, _)| dist_to_beacons.contains_key(f))
                .collect();
            if common.len() < 66 {
                continue;
            }
            for (f, (pair1, pair2)) in common {
                for &(norm_1, norm_2) in dist_to_beacons.get(f).unwrap() {
                    let rotation = ROTATIONS
                        .iter()
                        .find(|&r| norm_1 - (r * pair1) == norm_2 - (r * pair2));
                    if let Some(rotation) = rotation {
                        let normalized_vec = norm_1 - (rotation * pair1);
                        scanners.push(normalized_vec);
                        let normalized_list: Vec<_> = list
                            .iter()
                            .map(|&v| normalized_vec + (rotation * v))
                            .collect();
                        let mut matches = 0;
                        for b in normalized_list.iter() {
                            if normalized_beacons.contains(b) {
                                matches += 1;
                            }
                            if matches == 3 {
                                break;
                            }
                        }
                        if matches < 3 {
                            break;
                        }
                        normalized_beacons.extend(normalized_list);
                        let normalized_tags = tags.iter().map(|&(v, (p1, p2))| {
                            (
                                v,
                                (
                                    (normalized_vec + rotation * p1),
                                    (normalized_vec + rotation * p2),
                                ),
                            )
                        });
                        normalized_tags.into_iter().for_each(|(v, pair)| {
                            dist_to_beacons.entry(v).or_default().insert(pair);
                        });
                        to_remove = Some(idx);
                        break 'outer;
                    }
                }
            }
        }
        if let Some(idx) = to_remove {
            processed_lists.swap_remove(idx);
        }
    }
    println!("{}", normalized_beacons.len());
    let max_distance = scanners
        .iter()
        .tuple_combinations()
        .map(|(s1, s2)| {
            let vec = s2 - s1;
            vec[0].abs() + vec[1].abs() + vec[2].abs()
        })
        .max()
        .unwrap();
    println!("{}", max_distance);

    Ok(())
}
