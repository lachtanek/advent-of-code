use counter::Counter;

fn to_number(value: &str) -> u64 {
    value.parse::<u64>().unwrap()
}

fn split(stone: &u64) -> Option<(u64, u64)> {
    let n_digits = stone.checked_ilog10().unwrap_or(0) + 1;

    return if n_digits % 2 == 0 {
        let m = 10_u64.pow(n_digits / 2);
        Some((stone / m, stone % m))
    } else {
        None
    };
}

pub fn run(data: &String) {
    let (input, _) = data.split_once("\n").unwrap();
    let stones: Vec<u64> = input.split(" ").map(to_number).collect();

    let mut stones_25 = 0;
    let mut stone_counts: Counter<u64, u64> = Counter::new();

    for stone in stones {
        stone_counts[&stone] += 1;
    }

    for n in 0..75 {
        for (stone, n) in stone_counts.clone().iter() {
            if *stone == 0 {
                stone_counts[&1] += n;
                stone_counts[stone] -= n;
            } else if let Some((l, r)) = split(stone) {
                stone_counts[&l] += n;
                stone_counts[&r] += n;
                stone_counts[stone] -= n;
            } else {
                stone_counts[stone] -= n;
                stone_counts[&(*stone * 2024)] += n;
            }
        }

        if n == 24 {
            stones_25 = stone_counts.iter().map(|v| v.1).sum();
        }
    }

    let total: u64 = stone_counts.iter().map(|v| v.1).sum();
    println!("{:?}", (stones_25, total));
}
