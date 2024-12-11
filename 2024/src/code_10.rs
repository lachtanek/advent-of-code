use std::{collections::VecDeque, i32, ops};

type Map = Vec<Vec<i32>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Coords {
    x: i32,
    y: i32,
}

impl ops::Add<(i32, i32)> for Coords {
    type Output = Coords;

    fn add(self, other: (i32, i32)) -> Self::Output {
        Coords {
            x: self.x + other.0,
            y: self.y + other.1,
        }
    }
}

fn get(map: &Map, pos: Coords) -> Option<i32> {
    if let Ok(x) = usize::try_from(pos.x) {
        if let Ok(y) = usize::try_from(pos.y) {
            if let Some(x_vec) = map.get(y) {
                return x_vec.get(x).copied();
            }
        }
    }

    return None;
}

fn get_score(map: &Map, start: Coords) -> i32 {
    let mut score = 0;
    let mut stack: VecDeque<(Coords, i32)> = VecDeque::new();
    let mut visited: Vec<Coords> = Vec::new();
    stack.push_back((start, 0));

    while let Some((pos, height)) = stack.pop_front() {
        if visited.contains(&pos) {
            continue;
        }

        visited.push(pos);

        if height == 9 {
            score += 1;
            continue;
        }

        for d in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            if let Some(new_height) = get(map, pos + d) {
                if new_height == height + 1 {
                    stack.push_back((pos + d, new_height));
                }
            }
        }
    }

    return score;
}

fn get_rating(map: &Map, path: &Vec<Coords>, pos: Coords) -> Vec<Vec<Coords>> {
    let mut paths: Vec<Vec<Coords>> = Vec::new();
    let mut new_path = path.clone();
    new_path.push(pos);

    if let Some(height) = get(map, pos) {
        if height == 9 {
            paths.push(new_path.clone());
            return paths;
        }

        for direction in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            if let Some(new_height) = get(map, pos + direction) {
                if new_height == height + 1 {
                    let mut new_paths = get_rating(map, &new_path, pos + direction);
                    paths.append(&mut new_paths);
                }
            }
        }
    }

    return paths;
}

pub fn run(data: &String) {
    let map: Map = data
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| i32::try_from(c.to_digit(10).unwrap()).unwrap())
                .collect()
        })
        .collect();

    let mut trailheads: Vec<Coords> = Vec::new();
    for (y, xs) in map.iter().enumerate() {
        for (x, v) in xs.iter().enumerate() {
            if *v == 0 {
                trailheads.push(Coords {
                    x: i32::try_from(x).unwrap(),
                    y: i32::try_from(y).unwrap(),
                });
            }
        }
    }

    let total_score: i32 = trailheads.iter().map(|p| get_score(&map, *p)).sum();

    let mut total_rating = 0;
    for th in trailheads {
        let mut all_paths: Vec<Vec<Coords>> = get_rating(&map, &Vec::new(), th);
        all_paths.sort();
        all_paths.dedup();
        total_rating += all_paths.len();
    }

    println!("{:?}", total_score);
    println!("{:?}", total_rating);
}
