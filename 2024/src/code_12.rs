use std::ops;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    x: i32,
    y: i32,
}

struct Map2D {
    data: Vec<Vec<(char, bool)>>,
    size_x: i32,
    size_y: i32,
}

impl Map2D {
    fn _validate(&self, pos: Pos) -> Option<(usize, usize)> {
        if pos.x >= 0 && pos.x < self.size_x && pos.y >= 0 && pos.y < self.size_y {
            Some((
                usize::try_from(pos.x).unwrap(),
                usize::try_from(pos.y).unwrap(),
            ))
        } else {
            None
        }
    }

    fn get(&self, pos: Pos) -> Option<(char, bool)> {
        if let Some((x, y)) = self._validate(pos) {
            Some(self.data[y][x])
        } else {
            None
        }
    }

    fn set(&mut self, pos: Pos, value: (char, bool)) {
        if let Some((x, y)) = self._validate(pos) {
            self.data[y][x] = value;
        }
    }

    fn _explore_region(&mut self, start: Pos) -> Region {
        let mut items = Vec::new();
        let mut to_visit = vec![start];
        let (plant, _) = self.get(start).unwrap();

        while let Some(pos) = to_visit.pop() {
            items.push(pos);

            for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let new_pos = pos + dir;

                if let Some((pos_plant, explored)) = self.get(new_pos) {
                    if plant == pos_plant && !explored {
                        self.set(new_pos, (pos_plant, true));
                        to_visit.push(new_pos);
                    }
                }
            }
        }

        return Region { items, plant };
    }

    fn get_regions(&mut self) -> Vec<Region> {
        let mut regions = vec![];

        for y in 0..self.size_y {
            for x in 0..self.size_x {
                let value = self.get(Pos { x, y }).unwrap();

                if !value.1 {
                    self.set(Pos { x, y }, (value.0, true));
                    regions.push(self._explore_region(Pos { x, y }));
                }
            }
        }

        return regions;
    }
}

fn map(v: Vec<Vec<char>>) -> Map2D {
    let size_x = i32::try_from(v[0].len()).unwrap();
    let size_y = i32::try_from(v.len()).unwrap();

    return Map2D {
        data: v
            .into_iter()
            .map(|xs| xs.into_iter().map(|v| (v, false)).collect())
            .collect(),
        size_x,
        size_y,
    };
}

impl ops::Add<(i32, i32)> for Pos {
    type Output = Pos;

    fn add(self, other: (i32, i32)) -> Self::Output {
        Pos {
            x: self.x + other.0,
            y: self.y + other.1,
        }
    }
}

#[derive(Debug)]
struct Region {
    plant: char,
    items: Vec<Pos>,
}

fn fence_cost(garden: &Map2D, region: &Region) -> i32 {
    let area = i32::try_from(region.items.len()).unwrap();
    let mut perimeter = 0;

    for item in region.items.iter() {
        for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if let Some((plant, _)) = garden.get(*item + dir) {
                if plant != region.plant {
                    perimeter += 1;
                }
            } else {
                perimeter += 1;
            }
        }
    }

    return area * perimeter;
}

fn discounted_fence_cost(garden: &Map2D, region: &Region) -> i32 {
    let area = i32::try_from(region.items.len()).unwrap();
    let mut sides_l: Vec<Pos> = Vec::new();
    let mut sides_r: Vec<Pos> = Vec::new();
    let mut sides_t: Vec<Pos> = Vec::new();
    let mut sides_b: Vec<Pos> = Vec::new();

    for item in region.items.iter() {
        for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_pos = *item + dir;

            let sides: &mut Vec<_> = match dir {
                (0, 1) => &mut sides_b,
                (0, -1) => &mut sides_t,
                (1, 0) => &mut sides_r,
                (-1, 0) => &mut sides_l,
                _ => panic!(),
            };

            if let Some((plant, _)) = garden.get(new_pos) {
                if plant != region.plant {
                    sides.push(*item);
                }
            } else {
                sides.push(*item);
            }
        }
    }

    sides_l.sort_by_key(|v| (v.x, v.y));
    sides_r.sort_by_key(|v| (v.x, v.y));
    sides_t.sort_by_key(|v| (v.y, v.x));
    sides_b.sort_by_key(|v| (v.y, v.x));

    let mut n_sides = 0;

    for sides in [&sides_l, &sides_r] {
        let mut x = -2;
        let mut y = -2;

        for pos in sides.iter() {
            if y + 1 != pos.y || pos.x != x {
                n_sides += 1;
            }

            (x, y) = (pos.x, pos.y);
        }
    }

    for sides in [&sides_t, &sides_b] {
        let mut x = -2;
        let mut y = -2;

        for pos in sides.iter() {
            if x + 1 != pos.x || pos.y != y {
                n_sides += 1;
            }

            (x, y) = (pos.x, pos.y);
        }
    }

    return area * n_sides;
}

pub fn run(data: &String) {
    let mut garden = map(data.lines().map(|l| l.chars().collect()).collect());

    let regions = garden.get_regions();

    let cost: i32 = regions.iter().map(|r| fence_cost(&garden, r)).sum();
    let discounted_cost: i32 = regions
        .iter()
        .map(|r| discounted_fence_cost(&garden, r))
        .sum();

    println!("total fence cost {:?}", cost);
    println!("total discounted fence cost {:?}", discounted_cost);
}
