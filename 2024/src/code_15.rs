use std::ops;

use itertools::Itertools;

use crate::util::coords::Coords;

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
    // this didnt work for me idk why /shrug
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
                // println!("try move again {:?} {:?}", new_pos, dir);
                if self.move_box(new_pos, dir) {
                    self.set(box_pos, '.');
                    self.set(new_pos, 'O');
                    return true;
                } else {
                    return false;
                }
            }
            '#' => false,
            '.' => {
                self.set(box_pos, '.');
                self.set(new_pos, 'O');

                assert!(self[new_pos] == 'O');
                return true;
            }
            _ => panic!(),
        }
    }

    fn set(&mut self, pos: Coords, val: char) {
        let (x, y) = self._validate(pos).unwrap();
        self.data[y][x] = val;
    }

    fn _get_big_box(&self, pos: Coords) -> (Coords, Coords) {
        match self[pos] {
            '[' => (pos, pos + (1, 0)),
            ']' => (pos + (-1, 0), pos),
            _ => panic!(),
        }
    }

    fn can_move_big_box(&self, box_pos: Coords, dir: Coords) -> bool {
        match self[box_pos] {
            '[' | ']' => {
                let (box_left, box_right) = self._get_big_box(box_pos);

                if dir.y != 0 {
                    // vertical move
                    return self.can_move_big_box(box_left + dir, dir)
                        && self.can_move_big_box(box_right + dir, dir);
                } else {
                    if dir.x > 0 {
                        // right
                        return self.can_move_big_box(box_right + dir, dir);
                    } else {
                        // left
                        return self.can_move_big_box(box_left + dir, dir);
                    }
                }
            }
            '#' => false,
            '.' => true,
            _ => panic!(),
        }
    }

    fn is_box(&self, pos: Coords) -> bool {
        self.get(pos) == Some(&'[') || self.get(pos) == Some(&']')
    }

    fn move_big_box(&mut self, box_pos: Coords, dir: Coords) -> bool {
        if !self.can_move_big_box(box_pos, dir) {
            return false;
        }

        let (box_left, box_right) = self._get_big_box(box_pos);

        if dir.y != 0 {
            if self.is_box(box_left + dir) {
                self.move_big_box(box_left + dir, dir);
            }

            if self.is_box(box_right + dir) {
                self.move_big_box(box_right + dir, dir);
            }
        } else if dir.x > 0 {
            if self.is_box(box_right + dir) {
                self.move_big_box(box_right + dir, dir);
            }
        } else {
            if self.is_box(box_left + dir) {
                self.move_big_box(box_left + dir, dir);
            }
        }

        self.set(box_left, '.');
        self.set(box_right, '.');
        self.set(box_left + dir, '[');
        self.set(box_right + dir, ']');

        true
    }

    // fn print(&self, guard: Coords) {
    //     for y in 0..self.size_y {
    //         for x in 0..self.size_x {
    //             let pos = Coords { x, y };
    //             if pos == guard {
    //                 print!("@");
    //             } else {
    //                 print!("{}", self[pos]);
    //             }
    //         }
    //         println!();
    //     }
    //     println!();
    // }
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

fn get_big_map(map_in: &Map, guard_in: &Coords) -> (Map, Coords) {
    let mut map_data = Vec::new();

    for y in 0..usize::try_from(map_in.size_y).unwrap() {
        map_data.push(Vec::new());

        for x in 0..usize::try_from(map_in.size_x).unwrap() {
            match map_in.data[y][x] {
                'O' => {
                    map_data[y].push('[');
                    map_data[y].push(']');
                }
                ch => {
                    map_data[y].push(ch);
                    map_data[y].push(ch);
                }
            }
        }
    }

    return (
        Map::new(map_data),
        Coords {
            x: guard_in.x * 2,
            y: guard_in.y,
        },
    );
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
    let (mut big_map, mut big_guard) = get_big_map(&map, &guard);
    let mut instructions = Vec::new();

    for line in data.lines().skip_while(|l| !l.is_empty()) {
        instructions.append(&mut line.chars().collect_vec());
    }

    // part 1
    for instruction in instructions.iter() {
        let dir = get_direction(instruction);
        let new_guard = guard + dir;

        match map.get(new_guard) {
            Some('O') => {
                if map.move_box(new_guard, dir) {
                    guard = new_guard;
                }
            }
            Some('.') => guard = new_guard,
            _ => {}
        }
    }

    // part 2
    for instruction in instructions.iter() {
        let dir = get_direction(instruction);
        let new_guard = big_guard + dir;

        match big_map.get(new_guard) {
            Some('[') | Some(']') => {
                if big_map.move_big_box(new_guard, dir) {
                    big_guard = new_guard;
                }
            }
            Some('.') => big_guard = new_guard,
            _ => {}
        }

        // big_map.print(big_guard);
    }

    // part 1 result
    {
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

    // part 2 result
    {
        let mut total_gps = 0;
        for y in 0..big_map.size_y {
            for x in 0..big_map.size_x {
                let pos = Coords { x, y };

                if big_map[pos] == '[' {
                    total_gps += pos.gps();
                }
            }
        }

        println!("big gps: {:?}", total_gps);
    }
}
