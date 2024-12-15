use crate::util::coords::Coords;

fn to_number(value: &str) -> i32 {
    value.parse::<i32>().unwrap()
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    pos: Coords,
    vel: Coords,
}

fn parse_coords(inp: &str) -> Coords {
    // p=0,4
    let (_, c) = inp.split_once("=").unwrap();
    let (a, b) = c.split_once(",").unwrap();
    Coords {
        x: to_number(a),
        y: to_number(b),
    }
}

fn draw(robots: &Vec<Robot>, w: i32, h: i32) {
    for y in 0..h {
        for x in 0..w {
            let n = robots
                .iter()
                .filter(|r| r.pos.x == x && r.pos.y == y)
                .count();
            if n > 0 {
                print!("{}", n);
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn maybe_christmas_tree(robots: &mut Vec<Robot>) -> bool {
    robots.sort_by_key(|r| (r.pos.x, r.pos.y));
    let mut prev_pos = Coords { x: 0, y: 0 };
    let mut n_near = 0;
    for r in robots {
        if r.pos.dist(&prev_pos) < 2.0 {
            n_near += 1;
        } else {
            n_near = 0;
        }

        if n_near >= 20 {
            return true;
        }

        prev_pos = r.pos;
    }

    return false;
}

pub fn run(data: &String) {
    let mut robots = Vec::new();
    let (mut w, mut h) = (0, 0);

    for line in data.lines() {
        if w == 0 {
            let (l, r) = line.split_once("x").unwrap();
            (w, h) = (to_number(l), to_number(r));
            continue;
        }

        // p=0,4 v=3,-3
        let (l, r) = line.split_once(" ").unwrap();
        let robot = Robot {
            pos: parse_coords(l),
            vel: parse_coords(r),
        };

        robots.push(robot);
    }

    let (mut tl, mut tr, mut bl, mut br) = (0, 0, 0, 0);
    let (div_x, div_y) = (w / 2, h / 2);
    let mut prev_robots = robots.clone();

    for robot in robots {
        let new_pos = (robot.pos + (robot.vel * 100)).bound(w, h);

        if new_pos.x < div_x {
            // tl, bl
            if new_pos.y < div_y {
                tl += 1;
            } else if new_pos.y > div_y {
                bl += 1;
            }
        } else if new_pos.x > div_x {
            // tr, br
            if new_pos.y < div_y {
                tr += 1;
            } else if new_pos.y > div_y {
                br += 1;
            }
        }
    }

    for i in 0..10000 {
        prev_robots = prev_robots
            .iter()
            .map(|r| Robot {
                pos: (r.pos + (r.vel)).bound(w, h),
                vel: r.vel,
            })
            .collect();

        if maybe_christmas_tree(&mut prev_robots) {
            println!("iter {:?}:", i); // this should actually be i+1 and I failed on this -_-
            draw(&prev_robots, w, h);
        }
    }

    println!("safety factor: {:?}", tl * tr * bl * br);
}
