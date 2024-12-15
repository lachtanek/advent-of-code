use itertools::Itertools;

fn to_number(value: &str) -> f64 {
    value.parse::<f64>().unwrap()
}

fn parse_button(data: &str) -> (f64, f64) {
    // Button B: X+84, Y+37
    let (_, r) = data.split_once(": ").unwrap();
    let (xs, ys) = r.split_once(", ").unwrap();
    let (_, x) = xs.split_once("+").unwrap();
    let (_, y) = ys.split_once("+").unwrap();
    return (to_number(x), to_number(y));
}

fn parse_prize(data: &str) -> (f64, f64) {
    // Prize: X=7870, Y=6450
    let (_, r) = data.split_once(": ").unwrap();
    let (xs, ys) = r.split_once(", ").unwrap();
    let (_, x) = xs.split_once("=").unwrap();
    let (_, y) = ys.split_once("=").unwrap();
    return (to_number(x), to_number(y));
}

#[derive(Debug)]
struct ClawMachine {
    button_a: (f64, f64),
    button_b: (f64, f64),
    prize: (f64, f64),
}

fn get_solution(machine: &ClawMachine) -> Option<(i64, i64)> {
    let (a1, a2) = machine.button_a;
    let (b1, b2) = machine.button_b;
    let (s1, s2) = machine.prize;

    // not my proudest moment
    let y = (s2 - (a2 * s1) / a1) / (b2 - (a2 * b1) / a1);
    let x = (s1 - b1 * y) / a1;

    let (xr, yr) = (x.round(), y.round());
    if a1 * xr + b1 * yr == s1 && a2 * xr + b2 * yr == s2 {
        Some((xr as i64, yr as i64))
    } else {
        None
    }
}

fn big_machine(machine: &ClawMachine) -> ClawMachine {
    ClawMachine {
        button_a: machine.button_a,
        button_b: machine.button_b,
        prize: (
            machine.prize.0 + 10000000000000f64,
            machine.prize.1 + 10000000000000f64,
        ),
    }
}

pub fn run(data: &String) {
    let mut machines: Vec<_> = Vec::new();

    for mut lines in &data.lines().chunks(4) {
        let button_a = lines.next().unwrap();
        let button_b = lines.next().unwrap();
        let prize = lines.next().unwrap();
        lines.next(); // newline

        machines.push(ClawMachine {
            button_a: parse_button(button_a),
            button_b: parse_button(button_b),
            prize: parse_prize(prize),
        });
    }

    let mut cost = 0;
    let mut cost_big = 0;
    for machine in machines.iter() {
        if let Some(solution) = get_solution(machine) {
            cost += 3 * solution.0 + solution.1;
        }

        if let Some(big_solution) = get_solution(&big_machine(machine)) {
            cost_big += 3 * big_solution.0 + big_solution.1;
        }
    }

    println!("{:?}", cost);
    println!("{:?}", cost_big);
}
