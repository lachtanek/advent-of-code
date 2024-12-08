use std::{
    cmp::{max, min},
    ops,
};

use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Coords {
    x: i32,
    y: i32,
}

impl ops::Add<Coords> for Coords {
    type Output = Coords;
    fn add(self, other: Coords) -> Self::Output {
        Coords {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub<Coords> for Coords {
    type Output = Coords;
    fn sub(self, other: Coords) -> Self::Output {
        Coords {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::Neg for Coords {
    type Output = Coords;
    fn neg(self) -> Self::Output {
        Coords {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl ops::Mul<i32> for Coords {
    type Output = Coords;
    fn mul(self, other: i32) -> Self::Output {
        Coords {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Coords {
    fn abs(self) -> Coords {
        Coords {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

fn coords(x: i32, y: i32) -> Coords {
    Coords { x, y }
}

fn get_antennas_sorted(data: &String) -> (Vec<(char, Coords)>, Coords) {
    let mut map: Vec<(char, Coords)> = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for (y, line) in data.lines().enumerate() {
        if line.is_empty() {
            break;
        }

        for (x, freq) in line.chars().enumerate() {
            if freq != '.' && freq != '#' {
                let coords = Coords {
                    x: i32::try_from(x).unwrap(),
                    y: i32::try_from(y).unwrap(),
                };

                map.push((freq, coords));
            }

            max_x = i32::try_from(x).unwrap();
        }

        max_y = i32::try_from(y).unwrap();
    }

    map.sort_by_key(|v| v.0);
    return (map, coords(max_x + 1, max_y + 1));
}

fn get_anti_nodes(v1: &Coords, v2: &Coords) -> (Coords, Coords) {
    let (x1, x2, y1, y2) = (v1.x, v2.x, v1.y, v2.y);
    let dist = *v1 - *v2;

    if (x1 > x2 && y1 < y2) || (x1 < x2 && y1 > y2) {
        return (*v1 + dist, *v2 - dist);
    } else {
        let (min_x, min_y) = (min(x1, x2), min(y1, y2));
        let (max_x, max_y) = (max(x1, x2), max(y1, y2));

        return (
            coords(min_x, min_y) - dist.abs(),
            coords(max_x, max_y) + dist.abs(),
        );
    }
}

fn generate_anti_nodes(vec: &Coords, dist: Coords, size: &Coords) -> Vec<Coords> {
    let mut result: Vec<Coords> = Vec::new();

    for i in 0..size.x {
        let node = *vec + (dist * i);
        if is_in_bounds(node, size) {
            result.push(node);
        } else {
            break;
        }
    }

    return result;
}

fn get_resonant_anti_nodes(v1: &Coords, v2: &Coords, size: &Coords) -> Vec<Coords> {
    let (x1, x2, y1, y2) = (v1.x, v2.x, v1.y, v2.y);
    let dist = *v1 - *v2;

    if (x1 > x2 && y1 < y2) || (x1 < x2 && y1 > y2) {
        let mut anti_nodes: Vec<Coords> = Vec::new();
        anti_nodes.append(&mut generate_anti_nodes(v1, dist, size));
        anti_nodes.append(&mut generate_anti_nodes(v2, -dist, size));
        return anti_nodes;
    } else {
        let (min_x, min_y) = (min(x1, x2), min(y1, y2));
        let (max_x, max_y) = (max(x1, x2), max(y1, y2));
        let mut anti_nodes: Vec<Coords> = Vec::new();

        anti_nodes.append(&mut generate_anti_nodes(&coords(min_x, min_y), -dist, size));
        anti_nodes.append(&mut generate_anti_nodes(&coords(max_x, max_y), dist, size));

        return anti_nodes;
    }
}

fn is_in_bounds(v: Coords, size: &Coords) -> bool {
    v.x >= 0 && v.y >= 0 && v.x < size.x && v.y < size.y
}

pub fn run(data: &String) {
    let (map, size) = get_antennas_sorted(data);
    // let by_freq = group_by_freq(map);
    let mut anti_nodes: Vec<Coords> = Vec::new();
    let mut resonant_anti_nodes: Vec<Coords> = Vec::new();

    for (_freq, coords_list) in &map.iter().chunk_by(|v| v.0) {
        let vec: Vec<_> = coords_list.map(|v| v.1).collect();

        for combo in vec.iter().combinations(2) {
            let (left, right) = (combo[0], combo[1]);
            let (a1, a2) = get_anti_nodes(left, right);
            if is_in_bounds(a1, &size) {
                anti_nodes.push(a1);
            }
            if is_in_bounds(a2, &size) {
                anti_nodes.push(a2);
            }

            resonant_anti_nodes.append(&mut get_resonant_anti_nodes(left, right, &size));
        }
    }

    anti_nodes.sort();
    anti_nodes.dedup();

    resonant_anti_nodes.sort();
    resonant_anti_nodes.dedup();

    println!("{:?}", (anti_nodes.len(), size));
    println!("{:?}", (resonant_anti_nodes.len(), size));
}
