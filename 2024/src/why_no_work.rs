use std::ops;

use itertools::Itertools;

use crate::util::coords::Coords;

// type Map = Vec<Vec<char>>;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Map {
    data: Vec<Vec<char>>,
    size_x: i32,
    size_y: i32,
}

impl Coords {
    fn gps(&self) -> i32 {
        self.y * 100 + self.x
    }
}

impl ops::Index<Coords> for Map {
    type Output = char;

    fn index(&self, pos: Coords) -> &Self::Output {
        let (x, y) = self._validate(pos).unwrap();
        &self.data[y][x]
    }
}

impl ops::IndexMut<Coords> for Map {
    fn index_mut(&mut self, pos: Coords) -> &mut Self::Output {
        let (y, x) = self._validate(pos).unwrap();
        &mut self.data[y][x]
    }
}

impl Map {
    fn new(data: Vec<Vec<char>>) -> Self {
        let size_x = i32::try_from(data[0].len()).unwrap();
        let size_y = i32::try_from(data.len()).unwrap();

        return Map {
            data,
            size_x,
            size_y,
        };
    }

    fn get(&self, pos: Coords) -> Option<&char> {
        self._validate(pos).map(|(x, y)| &self.data[y][x])
    }

    fn _validate(&self, pos: Coords) -> Option<(usize, usize)> {
        if pos.x >= 0 && pos.x < self.size_x && pos.y >= 0 && pos.y < self.size_y {
            Some((
                usize::try_from(pos.x).unwrap(),
                usize::try_from(pos.y).unwrap(),
            ))
        } else {
            None
        }
    }

    fn move_box(&mut self, box_pos: Coords, dir: Coords) -> bool {
        let new_pos = box_pos + dir;

        assert!(self[box_pos] == 'O');

        match self[new_pos] {
            'O' => {
                println!("try move again {:?} {:?}", new_pos, dir);
                if self.move_box(new_pos, dir) {
                    self[box_pos] = '.';
                    self[new_pos] = 'O';
                    return true;
                } else {
                    return false;
                }
            }
            '#' => false,
            '.' => {
                println!("movedd");
                self[box_pos] = '.';
                println!("old {:?}", self[new_pos]);
                self[new_pos] = 'O';
                println!("new {:?}", self[new_pos]);

                assert!(self[new_pos] == 'O');
                return true;
            }
            _ => panic!(),
        }
    }

    fn print(&self, guard: Coords) {
        for y in 0..self.size_y {
            for x in 0..self.size_x {
                let pos = Coords { x, y };
                if pos == guard {
                    print!("@");
                } else {
                    print!("{}", self[pos]);
                }
            }
            println!();
        }
        println!();
    }
}

#[cfg(test)]
mod test {
    use crate::util::coords::Coords;

    use super::Map;

    #[test]
    fn basics() {
        let mut m = Map::new(vec![vec!['a', 'b'], vec!['c', 'd']]);

        assert!(m[Coords::new(0, 0)] == 'a');
        m[Coords::new(0, 0)] = 'x';
        assert!(m[Coords::new(0, 0)] == 'x');
        assert!(m.data[0][0] == 'x');
    }
}

fn get_map(data: Vec<&str>) -> (Map, Coords) {
    let mut map_data = Vec::new();
    let mut guard: Option<Coords> = None;

    for (y, line) in data.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if map_data.get(y).is_none() {
                map_data.push(Vec::new());
            }

            if ch == '@' {
                map_data[y].push('.');

                if guard.is_none() {
                    guard = Some(Coords {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                    });
                } else {
                    panic!("Multiple guards found");
                }
            } else {
                map_data[y].push(ch);
            }
        }
    }

    if let Some(guard_pos) = guard {
        return (Map::new(map_data), guard_pos);
    } else {
        panic!("Did not find guard");
    }
}

fn get_direction(i: &char) -> Coords {
    match i {
        '>' => Coords { x: 1, y: 0 },
        '<' => Coords { x: -1, y: 0 },
        '^' => Coords { x: 0, y: -1 },
        'v' => Coords { x: 0, y: 1 },
        _ => panic!(),
    }
}

pub fn run(data: &String) {
    let lines_it = data.lines().take_while(|l| !l.is_empty()).collect_vec();
    let (mut map, mut guard) = get_map(lines_it);
    let mut instructions = Vec::new();

    for line in data.lines().skip_while(|l| !l.is_empty()) {
        instructions.append(&mut line.chars().collect_vec());
    }

    for instruction in instructions.iter() {
        let dir = get_direction(instruction);
        let new_guard = guard + dir;

        match map.get(new_guard) {
            Some('O') => {
                println!(
                    "try move {:?} {:?} {:?}",
                    new_guard,
                    dir,
                    map.get(new_guard)
                );
                if map.move_box(new_guard, dir) {
                    guard = new_guard;
                }
            }
            Some('.') => guard = new_guard,
            _ => {}
        }

        map.print(guard);
    }

    let mut total_gps = 0;
    for y in 0..map.size_y {
        for x in 0..map.size_x {
            let pos = Coords { x, y };
            if map[pos] == 'O' {
                total_gps += pos.gps();
            }
        }
    }

    println!("{:?}", total_gps);
}
