use std::env;

mod guess_game;
mod matcher;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("사용법: cargo run <num>");
        return;
    }

    matcher::run(&args);
}
