use std::cmp::Ordering;

fn to_number(value: &str) -> i32 {
    value.parse::<i32>().unwrap()
}

fn is_print_correct(print_vec: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
    for (i, num) in print_vec.iter().enumerate() {
        for rule in rules {
            if rule.1 == *num {
                if let Some(found_idx) = print_vec.iter().position(|v| *v == rule.0) {
                    if found_idx > i {
                        return false;
                    }
                }
            }
        }
    }

    return true;
}

fn get_correct_print(print_vec: &Vec<i32>, rules: &Vec<(i32, i32)>) -> Vec<i32> {
    let mut new_vec = print_vec.clone();

    new_vec.sort_by(|a, b| {
        if rules.contains(&(*a, *b)) {
            Ordering::Less
        } else if rules.contains(&(*b, *a)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });

    return new_vec;
}

pub fn run(data: &String) {
    let mut rules = Vec::<(i32, i32)>::new();
    let mut prints = Vec::<Vec<i32>>::new();
    let mut parsing_rules = true;

    for line in data.lines() {
        if line.is_empty() {
            if parsing_rules {
                parsing_rules = false;
                continue;
            } else {
                break;
            }
        }

        if parsing_rules {
            let rule_data: Vec<i32> = line.split('|').map(to_number).collect();
            rules.push((rule_data[0], rule_data[1]));
        } else {
            let print_data: Vec<i32> = line.split(',').map(to_number).collect();
            prints.push(print_data);
        }
    }

    let mut result1 = 0;
    let mut result2 = 0;

    for print_vec in prints.iter() {
        if is_print_correct(print_vec, &rules) {
            result1 += print_vec[print_vec.len() / 2];
        } else {
            let new_print = get_correct_print(print_vec, &rules);
            result2 += new_print[new_print.len() / 2];
        }
    }

    println!("sum of middles in correct prints: {:?}", result1);
    println!("sum of middles in corrected prints: {:?}", result2);
}
