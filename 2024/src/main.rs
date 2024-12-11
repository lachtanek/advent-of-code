mod code_01;
mod code_02;
mod code_03;
mod code_04;
mod code_05;
mod code_06;
mod code_07;
mod code_08;
mod code_09;
mod code_10;
mod code_11;
use regex::Regex;
use std::env;

fn main() {
    let re = Regex::new(r"\d{2}").unwrap();
    let mut args = env::args();
    args.next();
    args.next();

    let file_name = args.next().unwrap();
    let file_content = std::fs::read_to_string(&file_name).unwrap();

    if let Some(m) = re.captures(file_name.as_str()) {
        match &m[0] {
            "01" => code_01::run(&file_content),
            "02" => code_02::run(&file_content),
            "03" => code_03::run(&file_content),
            "04" => code_04::run(&file_content),
            "05" => code_05::run(&file_content),
            "06" => code_06::run(&file_content),
            "07" => code_07::run(&file_content),
            "08" => code_08::run(&file_content),
            "09" => code_09::run(&file_content),
            "10" => code_10::run(&file_content),
            "11" => code_11::run(&file_content),
            _ => panic!("Undefined code"),
        }
    }
}
