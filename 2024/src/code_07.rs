#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Mul,
    Concat,
}

fn to_number(value: &str) -> i64 {
    value.parse::<i64>().unwrap()
}

// fujky, should be possible to do better, but /shrug
fn concat(left: i64, right: &i64) -> i64 {
    (left.to_string() + &right.to_string())
        .parse::<i64>()
        .unwrap()
}

fn compute_result(left: i64, op: Operator, others: &[i64], target: i64) -> bool {
    if let Some(right) = others.first() {
        let value = match op {
            Operator::Add => left + right,
            Operator::Mul => left * right,
            Operator::Concat => concat(left, right),
        };

        if value <= target {
            return compute_result(value, Operator::Add, &others[1..], target)
                || compute_result(value, Operator::Mul, &others[1..], target)
                || compute_result(value, Operator::Concat, &others[1..], target);
        } else {
            return false;
        }
    } else {
        return left == target;
    }
}

pub fn run(data: &String) {
    let mut sum = 0;

    for line in data.lines() {
        let mut data1 = line.split(": ");

        let result = to_number(data1.next().unwrap());
        let others = data1.next().unwrap().split(" ");

        let operands: Vec<_> = others.map(to_number).collect();
        let left = operands.first().unwrap();

        if compute_result(*left, Operator::Add, &operands[1..], result)
            || compute_result(*left, Operator::Mul, &operands[1..], result)
            || compute_result(*left, Operator::Concat, &operands[1..], result)
        {
            sum += result;
        }
    }

    println!("{:?}", (sum, i64::MAX));
}
