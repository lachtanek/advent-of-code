mod code_02;
use std::{env};

fn main() {
    let mut args = env::args();
    args.next();
    args.next();
    let fname = args.next().unwrap();

    // code_01::run_01();
    code_02::run_02(&fname);
}
