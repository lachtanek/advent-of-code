use std::ops;

type Matrix = Vec<Vec<char>>;

#[derive(Clone, Copy, Debug)]
struct Coords {
    x: usize,
    y: usize,
}

impl ops::Add<(i8, i8)> for Coords {
    type Output = Coords;

    fn add(self, other: (i8, i8)) -> Self::Output {
        Coords {
            x: self.x + other.0 as usize,
            y: self.y + other.1 as usize,
        }
    }
}

fn get(m: &Matrix, c: Coords) -> Option<char> {
    match m.get(c.y) {
        Some(v) => v.get(c.x).copied(),
        _ => None,
    }
}

fn move_in(value: (i8, i8), direction: (i8, i8)) -> (i8, i8) {
    (value.0 * direction.0, value.1 * direction.1)
}

fn find_xmas(m: &Matrix, c1: Coords) -> i32 {
    let directions = [
        (1, 0),
        (0, 1),
        (1, 1),
        (-1, 0),
        (0, -1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];
    let mut n = 0;

    for d in directions {
        let c2 = c1 + move_in((1, 1), d);
        let c3 = c1 + move_in((2, 2), d);
        let c4 = c1 + move_in((3, 3), d);

        //                 get(m, c1) == Some('X')
        let found = get(m, c2) == Some('M') && get(m, c3) == Some('A') && get(m, c4) == Some('S');

        if found {
            n += 1;
        }
    }

    return n;
}

fn find_x_mas(m: &Matrix, c1: Coords) -> i32 {
    let tl = c1 + (-1, -1);
    let tr = c1 + (1, -1);
    let bl = c1 + (-1, 1);
    let br = c1 + (1, 1);

    let found1 = get(m, tl) == Some('M') && get(m, br) == Some('S');
    let found2 = get(m, tl) == Some('S') && get(m, br) == Some('M');

    let found3 = get(m, tr) == Some('M') && get(m, bl) == Some('S');
    let found4 = get(m, tr) == Some('S') && get(m, bl) == Some('M');

    if (found1 || found2) && (found3 || found4) {
        return 1;
    }

    return 0;
}

pub fn run(data: &String) {
    let m: Matrix = data
        .lines()
        .filter(|l| l.len() > 0)
        .map(|l| l.chars().into_iter().collect())
        .collect();
    let mut total_xmas = 0;
    let mut total_x_mas = 0;

    for y in 0..m.len() {
        for x in 0..m[y].len() {
            if m[y][x] == 'X' {
                total_xmas += find_xmas(&m, Coords { x, y });
            }

            if m[y][x] == 'A' {
                total_x_mas += find_x_mas(&m, Coords { x, y });
            }
        }
    }

    println!("Found N xmas: {:?}", total_xmas);
    println!("Found N x-mas: {:?}", total_x_mas);
}
