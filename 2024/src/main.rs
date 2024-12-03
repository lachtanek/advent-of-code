mod code_03;
use std::env;

fn main() {
    let mut args = env::args();
    args.next();
    args.next();

    let fname = args.next().unwrap();
    let file_content = std::fs::read_to_string(fname).unwrap();

    // code_01::run_01();
    // code_02::run_02(&file_content);
    code_03::run_03(&file_content);
}
