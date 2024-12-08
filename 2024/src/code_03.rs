use regex::Regex;

fn to_number(value: &str) -> i32 {
    value.parse::<i32>().unwrap()
}

pub fn run(data: &String) {
    let re = Regex::new(r"(?:mul\((\d+),(\d+)\))|(?:do\(\))|(?:don't\(\))").unwrap();

    let mut total = 0;
    let mut total_enabled = 0;
    let mut enabled = true;

    for line in data.lines() {
        if line.is_empty() {
            continue;
        }

        let mut value = line;

        while let Some(m) = re.captures(value) {
            if &m[0] == "do()" {
                enabled = true;
            } else if &m[0] == "don't()" {
                enabled = false;
            } else {
                let mul = to_number(&m[1]) * to_number(&m[2]);
                total += mul;
                if enabled {
                    total_enabled += mul;
                }
            }

            let new_start = m.get(0).unwrap().end();
            value = &value[new_start..];
        }
    }

    println!("result1: {:?}", total);
    println!("result2: {:?}", total_enabled);
}
