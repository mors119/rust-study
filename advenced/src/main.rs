use std::env;

mod command;
mod files;
mod matcher;
mod network;

fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    log::info!("application started");

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        log::warn!("this is warn.");
        println!("사용법: cargo run <category> <number> [extra args...]");
        return;
    }

    matcher::run(&args);
}
