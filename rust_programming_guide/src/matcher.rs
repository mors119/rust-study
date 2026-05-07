use crate::common;
use crate::guess_game;

pub fn run(args: &[String]) {
    match args[1].as_str() {
        "guess" => guess_game::start::run(),
        "common" => match args[2].as_str() {
            "1" => common::common_1_variable::run(),
            "2" => common::common_2_scalar_type::run(),
            "3" => common::common_3_compound_type::run(),
            "4" => common::common_4_functions::run(),
            "5" => common::common_5_control_flow::run(),
            "6" => common::common_6_ownership::run(),
            "7" => common::common_7_struct::run(),
            "8" => common::common_8_enumeration::run(),
            _ => println!("basic에는 해당하는 번호가 없습니다."),
        },
        _ => println!("해당하는 번호가 없습니다."),
    }
}
