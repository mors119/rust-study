use crate::guess_game;

pub fn run(args: &[String]) {
    match args[1].as_str() {
        "1" => guess_game::start::run(),
        _ => println!("해당하는 번호가 없습니다."),
    }
}
