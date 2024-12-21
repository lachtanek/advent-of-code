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
mod code_12;
mod code_13;
mod code_14;
mod code_15;
mod code_16;
mod code_17;
mod code_18;
mod code_19;
mod code_20;
mod code_21;
mod code_22;
mod code_23;
mod code_24;
mod code_25;
pub mod util;
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
            "12" => code_12::run(&file_content),
            "13" => code_13::run(&file_content),
            "14" => code_14::run(&file_content),
            "15" => code_15::run(&file_content),
            "16" => code_16::run(&file_content),
            "17" => code_17::run(&file_content),
            "18" => code_18::run(&file_content),
            "19" => code_19::run(&file_content),
            "20" => code_20::run(&file_content),
            "21" => code_21::run(&file_content),
            "22" => code_22::run(&file_content),
            "23" => code_23::run(&file_content),
            "24" => code_24::run(&file_content),
            "25" => code_25::run(&file_content),
            _ => panic!("Undefined code"),
        }
    }
}
