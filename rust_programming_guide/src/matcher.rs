use crate::common;
use crate::guess_game;

pub fn run(args: &[String]) {
    match args[1].as_str() {
        "guess" => guess_game::start::run(),
        "common" => match args[2].as_str() {
            "1" => common::common_1_values::run(),
            _ => println!("basic에는 해당하는 번호가 없습니다."),
        },
        _ => println!("해당하는 번호가 없습니다."),
    }
}
