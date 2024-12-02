fn is_safe(row: Vec<i32>) -> bool {
    let differences: Vec<_> = row.iter()
        .zip(row.iter().skip(1))
        .map(|(v1, v2)| v2 - v1).collect();

    let all_increasing = differences.iter().all(|v| v > &0);
    let all_decreasing = differences.iter().all(|v| v < &0);
    let differences_ok = differences.iter().all(
        |v| v.abs() >= 1 && v.abs() <= 3
    );

    (all_increasing || all_decreasing) && differences_ok
}

pub fn run_02(fname: &String) {
    let data = std::fs::read_to_string(fname).unwrap();
    let mut n_safe = 0;
    let mut n_safe_2 = 0;

    for line in data.lines() {
        if line.is_empty() {
            continue;
        }

        let data = line.split_whitespace();
        let row: Vec<_> = data.map(|v| v.parse::<i32>().unwrap()).collect();

        if is_safe(row.clone()) {
            n_safe += 1;
        }

        let mut is_safe_removed = false;
        for to_remove in 0..row.len() {
            let fst = &row[0..to_remove];
            let lst = &row[to_remove+1..];
            let new_vec: Vec<i32> = fst.iter().chain(lst.iter()).map(|v| v.clone()).collect();

            if is_safe(new_vec) {
                is_safe_removed = true;
                break;
            }
        }

        if is_safe_removed {
            n_safe_2 += 1;
        }
    }

    println!("result1: {:?}", n_safe);
    println!("result2: {:?}", n_safe_2);
}
