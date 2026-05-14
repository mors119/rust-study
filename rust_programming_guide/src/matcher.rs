use crate::common;
use crate::guess_game;

pub fn run(args: &[String]) {
    match args[1].as_str() {
        "guess" => guess_game::start::run(),
        "common" => match args[2].as_str() {
            "1" => common::common_01_variable::run(),
            "2" => common::common_02_scalar_type::run(),
            "3" => common::common_03_compound_type::run(),
            "4" => common::common_04_functions::run(),
            "5" => common::common_05_control_flow::run(),
            "6" => common::common_06_ownership::run(),
            "7" => common::common_07_struct::run(),
            "8" => common::common_08_enumeration::run(),
            "9" => common::common_09_crate_package_module::run(),
            "10" => common::common_10_collection::run(),
            "11" => common::common_11_error_handling::run(),
            _ => println!("basic에는 해당하는 번호가 없습니다."),
        },
        _ => println!("해당하는 번호가 없습니다."),
    }
}
