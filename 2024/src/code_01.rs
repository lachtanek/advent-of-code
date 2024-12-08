use std::iter::zip;

pub fn run(fname: &String) {
    let data = std::fs::read_to_string(fname).unwrap();
    let mut values1 = Vec::<i32>::new();
    let mut values2 = Vec::<i32>::new();

    for line in data.lines() {
        if !line.is_empty() {
            let mut data = line.split_whitespace();
            let x1 = data.next().unwrap();
            let x2 = data.next().unwrap();
            let a = x1.parse::<i32>().unwrap();
            let b = x2.parse::<i32>().unwrap();
            values1.push(a);
            values2.push(b);
        }
    }

    let mut sorted1 = values1.clone();
    let mut sorted2 = values2.clone();
    sorted1.sort();
    sorted2.sort();
    let mut total = 0;
    for (v1, v2) in zip(sorted1, sorted2) {
        total += (v1 - v2).abs();
    }

    println!("result 1 {:?}", total);

    let mut score = 0;
    for current in values1.clone() {
        let n_times =
            values2.clone().into_iter().fold(
                0,
                |acc, value| {
                    if current == value {
                        acc + 1
                    } else {
                        acc
                    }
                },
            );

        score += current * n_times;
    }

    println!("result 2 {:?}", score);
}
