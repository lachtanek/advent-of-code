use itertools::Itertools;

fn build_pattern(pattern: &str, towels: &Vec<String>) -> i32 {
    let usable_towels = towels.iter().filter(|towel| pattern.starts_with(*towel));

    let possible_options = usable_towels.map(|towel| {
        if towel.len() >= pattern.len() {
            1
        } else {
            build_pattern(&pattern[towel.len()..], towels)
        }
    });

    possible_options.sum()
}

fn parse_towels(inp: &str) -> Vec<&str> {
    inp.split(", ").map(|v| v.to_owned()).collect_vec()
}

fn try_build_pattern<'a>(pattern: &str, towels: &Vec<&'a str>) -> Vec<Vec<&'a str>> {
    let usable_towels = towels
        .into_iter()
        .filter(|towel| pattern.starts_with(*towel));

    let possible_options = usable_towels.flat_map(|towel| {
        if towel.len() >= pattern.len() {
            vec![vec![*towel]]
        } else {
            try_build_pattern(&pattern[towel.len()..], towels)
                .into_iter()
                .map(|mut ts| {
                    ts.insert(0, towel);
                    ts
                })
                .collect_vec()
        }
    });

    possible_options.collect_vec()
}

pub fn run(data: &String) {
    let towels = data
        .lines()
        .next()
        .unwrap()
        .split(", ")
        .map(|v| v)
        .collect_vec();
    let patterns = data.lines().skip(2).map(|v| v.to_owned()).collect_vec();

    let mut base_towels = Vec::new();
    for (i, towel) in towels.iter().enumerate() {
        let mut new_towels = towels.clone();
        new_towels.remove(i);
        let buildable_from = try_build_pattern(towel, &new_towels);

        if buildable_from.is_empty() {
            base_towels.push(towel);
        } else {
            for ts in buildable_from {
                for t in ts {
                    base_towels.push(t);
                }
            }
        }
    }

    base_towels.sort();
    base_towels.dedup();
    println!("new towels: {:?}", base_towels);

    // let mut doable = 0;
    // let mut total = 0;
    // for pattern in patterns {
    //     let n_patterns = build_pattern(&pattern, &towels);
    //     total += n_patterns;
    //     println!("{:?} done", pattern);
    //     if n_patterns > 0 {
    //         doable += 1;
    //     }
    // }

    // println!("doable patterns: {:?}", doable);
}
