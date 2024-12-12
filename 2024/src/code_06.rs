use std::ops;

type Map = Vec<Vec<char>>;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

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

fn try_move(map: &Map, position: Coords, direction: Direction) -> Option<(Coords, char)> {
    let new_position = match direction {
        Direction::Left => position + (-1, 0),
        Direction::Right => position + (1, 0),
        Direction::Up => position + (0, -1),
        Direction::Down => position + (0, 1),
    };

    if let Ok(x) = usize::try_from(new_position.x) {
        if let Ok(y) = usize::try_from(new_position.y) {
            if let Some(x_vec) = map.get(y) {
                if let Some(content) = x_vec.get(x) {
                    return Some((new_position, *content));
                }
            }
        }
    }

    return None;
}

fn turn_right(direction: Direction) -> Direction {
    match direction {
        Direction::Left => Direction::Up,
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
    }
}

fn get_map(data: &String) -> (Map, Coords) {
    let mut map: Map = Vec::new();
    let mut guard: Option<Coords> = None;

    for (y, line) in data.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if map.get(y).is_none() {
                map.push(Vec::new());
            }

            map[y].push(ch);

            if ch == '^' {
                if guard.is_none() {
                    guard = Some(Coords {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                    });
                } else {
                    panic!("Multiple guards found");
                }
            }
        }
    }

    if let Some(guard_pos) = guard {
        return (map, guard_pos);
    } else {
        panic!("Did not find guard");
    }
}

fn test_fake_wall(
    map: &Map,
    fake_wall: Coords,
    mut position: Coords,
    mut direction: Direction,
) -> bool {
    let mut seen_walls = vec![];

    while let Some((new_pos, content)) = try_move(map, position, direction) {
        if content == '#' || new_pos == fake_wall {
            // looping, break out
            if seen_walls.contains(&(new_pos, direction)) {
                return true;
            }

            seen_walls.push((new_pos, direction));
            direction = turn_right(direction);
        } else {
            position = new_pos;
        }
    }

    return false;
}

pub fn run(data: &String) {
    let (map, start) = get_map(data);
    let mut position = start;
    let mut direction = Direction::Up;
    let mut visited = vec![position];
    let mut possible_fake_walls: Vec<Coords> = Vec::new();

    while let Some((new_pos, content)) = try_move(&map, position, direction) {
        if content == '#' {
            direction = turn_right(direction);
        } else {
            // try placing a fake wall on next position
            if test_fake_wall(&map, new_pos, start, Direction::Up) {
                possible_fake_walls.push(new_pos);
            }

            position = new_pos;
            visited.push(position);
        }
    }

    visited.sort();
    visited.dedup();

    possible_fake_walls.sort();
    possible_fake_walls.dedup();

    println!("visited unique spaces: {:?}", visited.len());
    println!("{:?}", possible_fake_walls.contains(&start));
    println!(
        "unique possible fake walls: {:?}",
        possible_fake_walls.len()
    );
}
