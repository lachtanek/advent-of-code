fn to_number(value: &str) -> u64 {
    value.parse::<u64>().unwrap()
}

// const ZERO_EVOLUTION_SIZE: usize = 49;
// const ZERO_EVOLUTION: [u64; ZERO_EVOLUTION_SIZE] = [
//     1, 1, 2, 4, 4, 7, 14, 16, 20, 39, 62, 81, 110, 200, 328, 418, 667, 1059, 1546, 2377, 3572,
//     5602, 8268, 12343, 19778, 29165, 43726, 67724, 102131, 156451, 234511, 357632, 549949, 819967,
//     1258125, 1916299, 2886408, 4414216, 6669768, 10174278, 15458147, 23333796, 35712308, 54046805,
//     81997335, 125001266, 189148778, 288114305, 437102505,
// ];

fn count_stones(stone: u64, n: usize) -> u64 {
    if n == 0 {
        return 1;
    }

    if stone == 0 {
        // [4, 0, 4, 8, 20, 24, 4, 0, 4, 8, 8, 0, 9, 6]
        if n > 7 {
            return count_stones(4, n - 7)
                + count_stones(0, n - 7)
                + count_stones(4, n - 7)
                + count_stones(8, n - 7)
                + count_stones(20, n - 7)
                + count_stones(24, n - 7)
                + count_stones(4, n - 7)
                + count_stones(0, n - 7)
                + count_stones(4, n - 7)
                + count_stones(8, n - 7)
                + count_stones(8, n - 7)
                + count_stones(0, n - 7)
                + count_stones(9, n - 7)
                + count_stones(6, n - 7);
        } else if n > 4 {
            return count_stones(2, n - 4)
                + count_stones(0, n - 4)
                + count_stones(2, n - 4)
                + count_stones(4, n - 4);
        } else {
            return [1, 1, 2, 4][n - 1];
        }
    } else {
        let n_digits = stone.checked_ilog10().unwrap_or(0) + 1;

        // if n_digits == 4 && n >= 2 {
        //     // XXXX
        //     // XX XX
        //     // X X X X
        //     return count_stones(stone / 1000, n - 2)
        //         + count_stones((stone % 1000) / 100, n - 2)
        //         + count_stones((stone % 100) / 10, n - 2)
        //         + count_stones(stone % 10, n - 2);
        // }
        if n_digits % 2 == 0 {
            let m = 10_u64.pow(n_digits / 2);
            let (l, r) = (stone / m, stone % m);
            return count_stones(l, n - 1) + count_stones(r, n - 1);
        } else {
            return count_stones(stone * 2024, n - 1);
        }
    }
}

pub fn run(data: &String) {
    let (input, _) = data.split_once("\n").unwrap();
    let mut stones: Vec<u64> = input.split(" ").map(to_number).collect();

    for n in 0..40 {
        println!("{:?} {:?}", n, count_stones(0, n));
    }
    let n = 10;
    let result: u64 = stones.iter().map(|s| count_stones(*s, n)).sum();
    println!("n={:?} : {:?} stones", n, result);

    for _ in 0..n {
        stones = stones
            .iter()
            .flat_map(|stone| {
                let digits = stone.to_string();
                let n_digits = digits.len();
                if *stone == 0 {
                    return vec![1];
                } else if n_digits % 2 == 0 {
                    return vec![
                        to_number(&digits[..n_digits / 2]),
                        to_number(&digits[n_digits / 2..]),
                    ];
                } else {
                    return vec![*stone * 2024];
                }
                // 0 => [1],
                // _ => [stone * 1024, 1]
            })
            .collect();

        // println!("{:?} {:?}MB", n, stones.len() * 8 / 1024 / 1024);
        println!("{:?}", stones);
    }

    // println!("{:?}", stones.len());
}
